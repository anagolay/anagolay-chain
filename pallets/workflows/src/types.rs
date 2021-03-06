// This file is part of Anagolay Foundation.

// Copyright (C) 2019-2022 Anagolay Foundation.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use anagolay_support::{
  AnagolayRecord, AnagolayStructure, AnagolayStructureData, AnagolayStructureExtra, AnagolayVersionData,
  AnagolayVersionExtra, ArtifactType, Characters, CreatorId, ForWhat, VersionId, WasmArtifactSubType,
};
use codec::{Decode, Encode};
use sp_runtime::RuntimeDebug;
use sp_std::{clone::Clone, collections::btree_map::BTreeMap, default::Default, vec, vec::Vec};

/// Definition of an Operation to execute in a Workflow Segment
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct OperationVersionReference {
  /// The Version id of the Operation to execute
  pub version_id: VersionId,
  /// The map representing the Operation configuration to apply upon execution
  pub config: BTreeMap<Characters, Characters>,
}

/// Contains a sequence of Operations, the eventual configuration of each one
/// of them, and a reference to the input required to bootstrap the process. In fact, the required
/// input may come from other Segments of the Workflow or from external input as well (eg: end-user
/// interaction)
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct WorkflowSegment {
  /// The collection of inputs for this segment, where a number lesser than zero means that the
  /// input must be acquired from the outside world (e.g.: user interaction) rather then from a
  /// precedently executed Workflow Segment (thus, its index)
  pub inputs: Vec<i8>,
  /// The sequence of operations to execute in this Segment
  pub sequence: Vec<OperationVersionReference>,
}

/// Workflow Data, used to generate `manifest.id`
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct WorkflowData {
  /// Human readable Workflow name. min 8, max 128(0.12kb) characters, slugify to use _
  pub name: Characters,
  /// Identifier of the creator users or system as a reference to his account id on the blockchain,
  /// pgp key or email
  pub creators: Vec<CreatorId>,
  /// Description can be markdown but not html. min 8, max 1024(1kb) chars
  pub description: Characters,
  /// Tells which groups the Workflow belongs to
  pub groups: Vec<ForWhat>,
  /// A list of Segment definitions
  pub segments: Vec<WorkflowSegment>,
}

impl AnagolayStructureData for WorkflowData {
  fn validate(&self) -> Result<(), Characters> {
    if self.name.len() < 8 || self.name.len() > 128 {
      Err("WorkflowData.name: length must be between 8 and 128 characters".into())
    } else if self.description.len() < 8 || self.description.len() > 1024 {
      Err("WorkflowData.description: length must be between 4 and 1024 characters".into())
    } else if self.name.len() < 8 || self.name.len() > 128 {
      Err("WorkflowData.name: length must be between 4 and 128 characters".into())
    } else if self.creators.len() != 1 {
      Err("WorkflowData.creators: only Workflows with a single creator are supported at the moment".into())
    } else {
      Ok(())
    }
  }
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct WorkflowExtra {}
impl AnagolayStructureExtra for WorkflowExtra {}
impl Default for WorkflowExtra {
  fn default() -> Self {
    WorkflowExtra {}
  }
}

impl Default for WorkflowData {
  fn default() -> Self {
    WorkflowData {
      name: "".into(),
      creators: vec!["".into()],
      description: "".into(),
      groups: vec![ForWhat::default()],
      segments: vec![],
    }
  }
}

pub type Workflow = AnagolayStructure<WorkflowData, WorkflowExtra>;

/// Storage record type
pub type WorkflowRecord<T> =
  AnagolayRecord<Workflow, <T as frame_system::Config>::AccountId, <T as frame_system::Config>::BlockNumber>;

/// Workflow Version artifact types. This enum corresponds to the different types of
/// packages created by the publisher service when an Workflow Version is published
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub enum WorkflowArtifactType {
  /// This refers to the documentation generated by the `cargo docs`. The entry point is predictable
  /// and always will be in following format `${ipfs_cid}/${manifest.data.name}/index.html`
  Docs,
  /// Git source code of the workflow, to be used as dependency
  Git,
  /// Wasm artifacts built by the wasm-pack. They are split in subtypes where every type contains
  /// the same wasm file, and also includes the various `.js` and `.d.ts` files to increase
  /// developers experience
  Wasm(WasmArtifactSubType),
}

impl ArtifactType for WorkflowArtifactType {}

/// Alias for the data type of the Workflow version
pub type WorkflowVersionData = AnagolayVersionData<WorkflowArtifactType>;

/// `WorkflowVersion` type, alias of [`AnagolayStructure<WorkflowVersionData,
/// AnagolayVersionExtra>`]
pub type WorkflowVersion = AnagolayStructure<WorkflowVersionData, AnagolayVersionExtra>;

/// This is the Storage record of Operation Version.
pub type WorkflowVersionRecord<T> =
  AnagolayRecord<WorkflowVersion, <T as frame_system::Config>::AccountId, <T as frame_system::Config>::BlockNumber>;
