// Copyright 2023 Pants project contributors (see CONTRIBUTORS.md).
// Licensed under the Apache License, Version 2.0 (see LICENSE).
use std::hash::{Hash, Hasher};

use crate::gen::pants::cache::JavascriptInferenceMetadata;
use crate::gen::pants::cache::dependency_inference_request::Metadata;

impl Hash for JavascriptInferenceMetadata {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.package_root.hash(state);
        self.config_root.hash(state);
        for pattern in &self.import_patterns {
            pattern.pattern.hash(state);
            pattern.replacements.hash(state);
        }
        for path in &self.paths {
            path.pattern.hash(state);
            path.replacements.hash(state);
        }
    }
}

impl Hash for Metadata {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Metadata::Js(m) => m.hash(state),
        }
    }
}
