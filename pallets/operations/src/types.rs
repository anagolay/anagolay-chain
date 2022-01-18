// This file is part of Anagolay Foundation.

// Copyright (C) 2019-2021 Anagolay Foundation.
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

use std::collections::BTreeMap;
// use super::*;
use anagolay::{
  AnagolayStructure, AnagolayStructureData, AnagolayStructureExtra, ForWhat, GenericId, Text,
};
use codec::{Decode, Encode};
use sp_runtime::RuntimeDebug;
use sp_std::{clone::Clone, default::Default, vec, vec::Vec};

/// Textual representation of a type
pub type TypeName = Vec<u8>;

/// Operation structure. This contains all the needed parameters which define the operation.
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct OperationData {
  /// max 128(0.12kb) characters, slugify to use _
  name: Text,
  /// max 512(0.5kb) or 1024(1kb) chars, can be markdown but not html
  description: Text,
  /// What operation accepts in the implementation. these are the params of the function with the types
  input: Vec<TypeName>,
  /// A map where keys are names of configuration parameters and values are collections of strings representing allowed values
  config: BTreeMap<Text, Vec<Text>>,
  /// A switch used to generate the Workflow segments  
  groups: Vec<ForWhat>,
  /// Data type name defining the operation output
  output: TypeName,
  /// The fully qualified URL for the repository, this can be any public repo URL
  repository: Text,
  /// Short name of the license, like "Apache-2.0"
  license: Text,
}

impl Default for OperationData {
  fn default() -> Self {
    OperationData {
      name: vec![],
      description: vec![],
      input: vec![],
      config: BTreeMap::new(),
      groups: vec![],
      output: vec![],
      repository: vec![],
      license: vec![],
    }
  }
}

impl AnagolayStructureData for OperationData {}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct OperationExtra {}
impl AnagolayStructureExtra for OperationExtra {}

pub type Operation = AnagolayStructure<OperationData, OperationExtra>;

pub enum PackageType {
  Crate,
  Wasm,
  Esm,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct OperationVersionPackage {
  package_type: PackageType,
  file_url: Text,
  ipfs_cid: GenericId,
}

/// Operation Version structure. This contains all the needed parameters which define the operation version.
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct OperationVersionData {
  operation_id: GenericId,
  parent_id: GenericId,
  rehosted_repo: Text,
  packages: Vec<OperationVersionPackage>,
}

impl Default for OperationVersionData {
  fn default() -> Self {
    OperationVersionData
  }
}

impl AnagolayStructureData for OperationVersionData {}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct OperationVersionExtra {
  created_at: u64,
}
impl AnagolayStructureExtra for OperationVersionExtra {}

pub type OperationVersion = AnagolayStructure<OperationVersionData, OperationVersionExtra>;
