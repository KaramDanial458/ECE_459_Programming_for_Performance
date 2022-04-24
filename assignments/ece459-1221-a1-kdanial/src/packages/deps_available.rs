use rpkg::debversion;
use rpkg::debversion::DebianVersionNum;

use crate::Packages;
use crate::packages::Dependency;
use crate::packages::VersionRelation;

impl Packages {
    /// Gets the dependencies of package_name, and prints out whether they are satisfied (and by which library/version) or not.
    pub fn deps_available(&self, package_name: &str) {
        if !self.package_exists(package_name) {
            println!("no such package {}", package_name);
            return;
        }

        let package_num = self.get_package_num(package_name);
        let dependency = &self.dependencies[package_num];

        println!("Package {}:", package_name);
        for dep in dependency {
            let satisfied_dependency = self.dep_is_satisfied(dep);
            let dep_string: String = self.dep2str(dep);
            if satisfied_dependency != None {
                let deb_num: Option<&DebianVersionNum> = self.get_installed_debver(satisfied_dependency.unwrap());
                println!("- dependency \"{}\"", dep_string);
                match deb_num {
                    Some(p) => println!("+ {} satisfied by installed version {}", satisfied_dependency.unwrap(), p),
                    None => println!("+ {} satisfied", satisfied_dependency.unwrap()),
                }
            } else {
                println!("- dependency \"{}\"", dep_string);
                println!("-> not satisfied");
            }
        }
    }

    /// Returns Some(package) which satisfies dependency dd, or None if not satisfied.
    pub fn dep_is_satisfied(&self, dd:&Dependency) -> Option<&str> {
        // presumably you should loop on dd
        for package in dd {
            if self.installed_debvers.contains_key(&package.package_num) {
                if package.rel_version.is_none() {
                    let installed_package_name = self.get_package_name(package.package_num);
                    return Some(installed_package_name);

                } else {
                    let installed_package_name = self.get_package_name(package.package_num);
                    let installed_deb_version = self.get_installed_debver(installed_package_name).unwrap();
                    let (version_relation, string_relation) = package.rel_version.as_ref().unwrap();

                    let dependency_deb_version = string_relation.parse::<debversion::DebianVersionNum>().unwrap();
                    let satisfied = debversion::cmp_debversion_with_op(&version_relation, installed_deb_version, &dependency_deb_version);

                    if satisfied == true {
                        return Some(installed_package_name);
                    }
                }
            }
        }
        return None;
    }

    /// Returns a Vec of packages which would satisfy dependency dd but for the version.
    /// Used by the how-to-install command, which calls compute_how_to_install().
    pub fn dep_satisfied_by_wrong_version(&self, dd:&Dependency) -> Vec<&str> {
        assert! (self.dep_is_satisfied(dd).is_none());
        let mut result = vec![];
        // another loop on dd
        for package in dd{
            let installed_package_name = self.get_package_name(package.package_num);
            let installed_deb_version = self.get_installed_debver(installed_package_name);

            if installed_deb_version != None {
                let (version_relation, string_relation) = package.rel_version.as_ref().unwrap();
                let dependency_deb_version = string_relation.parse::<debversion::DebianVersionNum>().unwrap();
                let satisfied = debversion::cmp_debversion_with_op(&version_relation, installed_deb_version.unwrap(), &dependency_deb_version);

                if !satisfied == true {
                    result.push(installed_package_name);
                }
            }
        }

        return result;
    }
}

