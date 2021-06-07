use std::{collections::HashMap, path::Path};
use cargo_toml::{
    Badges, Dependency, DependencyDetail, DepsSet, Edition, FeatureSet,
    Manifest, Package, PatchSet, Product, Profiles, Publish, Resolver,
    TargetDepsSet, Workspace,
};
use rune_syntax::hir::Rune;

use crate::{GitSpecifier, RuneProject};

// Generate the `Cargo.toml` manifest.
pub(crate) fn generate(
    rune: &Rune,
    name: &str,
    project: &RuneProject,
    current_dir: &Path,
) -> Manifest {
    let product = Product {
        path: Some("lib.rs".to_string()),
        edition: Some(Edition::E2018),
        crate_type: Some(vec!["cdylib".to_string()]),
        ..Default::default()
    };

    Manifest {
        package: Some(package(name)),
        lib: Some(product),
        dependencies: dependencies(rune, project, current_dir),
        workspace: Some(Workspace {
            members: vec![String::from(".")],
            default_members: vec![String::from(".")],
            ..Default::default()
        }),
        ..empty_manifest()
    }
}

fn package(name: &str) -> Package {
    Package {
        name: name.into(),
        edition: Edition::E2018,
        version: String::from("0.0.0"),
        publish: Publish::Flag(false),
        resolver: Some(Resolver::V2),
        ..empty_package()
    }
}

fn dependencies(
    rune: &Rune,
    project: &RuneProject,
    current_dir: &Path,
) -> DepsSet {
    let mut deps = DepsSet::new();

    // We always need the log crate
    let log = Dependency::Detailed(DependencyDetail {
        version: Some(String::from("0.4")),
        features: vec![
            String::from("max_level_debug"),
            String::from("release_max_level_info"),
        ],
        ..empty_dependency_detail()
    });
    deps.insert(String::from("log"), log);

    deps.insert(
        String::from("runic-types"),
        Dependency::Detailed(rune_project_dependency("runic-types", project)),
    );
    // hard-code the "runicos/base" image
    deps.insert(
        String::from("runicos-base-wasm"),
        Dependency::Detailed(rune_project_dependency(
            "images/runicos-base/wasm",
            project,
        )),
    );

    let proc_blocks: HashMap<_, _> = rune
        .proc_blocks()
        .map(|(_, pb)| (pb.name(), &pb.path))
        .collect();

    for (name, path) in proc_blocks {
        let dep = proc_block_dependency(name, path, project, current_dir);
        deps.insert(name.to_string(), Dependency::Detailed(dep));
    }

    deps
}

fn proc_block_dependency(
    name: &str,
    path: &rune_syntax::ast::Path,
    project: &RuneProject,
    current_dir: &Path,
) -> DependencyDetail {
    if is_builtin(path) {
        return builtin_proc_block(name, path, project);
    } else if path.base.starts_with(".") {
        return local_proc_block(path, current_dir);
    }

    if path.sub_path.is_none() && !path.base.contains("/") {
        if let Some(version) = &path.version {
            // it's from crates.io
            return DependencyDetail {
                version: Some(version.clone()),
                ..empty_dependency_detail()
            };
        }
    }

    // fall back to using git
    let repo = format!("https://github.com/{}.git", path.base);

    DependencyDetail {
        git: Some(repo),
        ..empty_dependency_detail()
    }
}

fn local_proc_block(
    path: &rune_syntax::ast::Path,
    current_dir: &Path,
) -> DependencyDetail {
    DependencyDetail {
        path: Some(current_dir.join(&path.base).display().to_string()),
        ..empty_dependency_detail()
    }
}

fn builtin_proc_block(
    name: &str,
    path: &rune_syntax::ast::Path,
    project: &RuneProject,
) -> DependencyDetail {
    match project {
        RuneProject::Disk(root_dir) => {
            let path = path.sub_path.as_deref().unwrap_or(name);

            DependencyDetail {
                path: Some(root_dir.join(path).display().to_string()),
                ..empty_dependency_detail()
            }
        },
        RuneProject::Git { repo, specifier } => git_dependency(repo, specifier),
    }
}

fn is_builtin(path: &rune_syntax::ast::Path) -> bool {
    path.base == "hotg-ai/rune"
}

fn rune_project_dependency(
    name: &str,
    project: &RuneProject,
) -> DependencyDetail {
    match project {
        RuneProject::Disk(root_dir) => {
            let path = root_dir.join(name);

            DependencyDetail {
                path: Some(path.display().to_string()),
                ..empty_dependency_detail()
            }
        },
        RuneProject::Git { repo, specifier } => git_dependency(repo, specifier),
    }
}

fn git_dependency(repo: &str, specifier: &GitSpecifier) -> DependencyDetail {
    let mut detail = DependencyDetail {
        git: Some(repo.to_string()),
        ..empty_dependency_detail()
    };

    match specifier.clone() {
        crate::GitSpecifier::Commit(rev) => detail.rev = Some(rev),
        crate::GitSpecifier::Tag(tag) => detail.tag = Some(tag),
        crate::GitSpecifier::Branch(branch) => detail.branch = Some(branch),
    }

    detail
}

fn empty_manifest() -> Manifest {
    Manifest {
        package: None,
        dependencies: DepsSet::default(),
        lib: None,
        workspace: None,
        dev_dependencies: DepsSet::default(),
        build_dependencies: DepsSet::default(),
        target: TargetDepsSet::default(),
        features: FeatureSet::default(),
        patch: PatchSet::default(),
        profile: Profiles::default(),
        badges: Badges::default(),
        bin: Vec::default(),
        bench: Vec::default(),
        test: Vec::default(),
        example: Vec::default(),
    }
}

fn empty_package() -> Package {
    Package {
        name: String::default(),
        edition: Edition::default(),
        version: String::default(),
        build: None,
        workspace: None,
        authors: Default::default(),
        links: None,
        description: None,
        homepage: None,
        documentation: None,
        readme: None,
        keywords: Vec::new(),
        categories: Vec::new(),
        license: None,
        license_file: None,
        repository: None,
        metadata: None,
        default_run: None,
        autobins: false,
        autoexamples: false,
        autotests: false,
        autobenches: false,
        publish: Publish::default(),
        resolver: None,
    }
}

fn empty_dependency_detail() -> DependencyDetail {
    DependencyDetail {
        version: None,
        registry: None,
        registry_index: None,
        path: None,
        git: None,
        branch: None,
        tag: None,
        rev: None,
        features: Vec::new(),
        optional: false,
        default_features: None,
        package: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_dependencies() {
        let rune = Rune::default();
        let project = RuneProject::default();

        let got = dependencies(&rune, &project, Path::new("."));

        assert_eq!(got.len(), 3);
        assert!(got.contains_key("log"));
        assert!(got.contains_key("runic-types"));
        assert!(got.contains_key("runicos-base-wasm"));
    }

    #[test]
    fn builtin_proc_block() {
        let repo = RuneProject::GITHUB_REPO;
        let commit = "asdf";
        let project = RuneProject::Git {
            repo: repo.to_string(),
            specifier: GitSpecifier::Commit(commit.to_string()),
        };
        let path = "hotg-ai/rune#proc_blocks/modulo".parse().unwrap();
        let should_be = DependencyDetail {
            git: Some(repo.to_string()),
            rev: Some(commit.to_string()),
            ..empty_dependency_detail()
        };

        let got =
            proc_block_dependency("normalize", &path, &project, Path::new("."));

        assert_eq!(got, should_be);
    }

    #[test]
    fn external_proc_block() {
        let project = RuneProject::default();
        let path = "whatever@1.2".parse().unwrap();
        let should_be = DependencyDetail {
            version: Some("1.2".to_string()),
            ..empty_dependency_detail()
        };

        let got =
            proc_block_dependency("whatever", &path, &project, Path::new("."));

        assert_eq!(got, should_be);
    }

    #[test]
    fn manifest_generates_cdylib() {
        let project = RuneProject::default();
        let rune = Rune::default();

        let got = generate(&rune, "foo", &project, Path::new("."));

        let crate_type = got.lib.unwrap().crate_type.unwrap();
        assert!(crate_type.contains(&String::from("cdylib")));
    }

    #[test]
    fn manifest_is_in_its_own_workspace() {
        let project = RuneProject::default();
        let rune = Rune::default();

        let got = generate(&rune, "foo", &project, Path::new("."));

        assert!(got.workspace.is_some());
    }
}
