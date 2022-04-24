use crate::Packages;
use crate::packages::Dependency;
use std::collections::HashSet;
use crate::packages::DebianVersionNum;
use rpkg::debversion;

impl Packages {
    /// Computes a solution for the transitive dependencies of package_name; when there is a choice A | B | C, 
    /// chooses the first option A. Returns a Vec<i32> of package numbers.
    ///
    /// Note: does not consider which packages are installed.
    pub fn transitive_dep_solution(&self, package_name: &str) -> Vec<i32> {
        if !self.package_exists(package_name) {
            return vec![];
        }

        let deps : &Vec<Dependency> = &*self.dependencies.get(self.get_package_num(package_name)).unwrap();
        let mut dependency_set = vec![];
        let mut dependencies_visited = HashSet::new();
        let mut counter = 0;

        // implement worklist
        for depend in deps {
            let first_depend = depend.first().unwrap();
            dependency_set.push(first_depend.package_num);
            dependencies_visited.insert(first_depend.package_num);
        }

        while counter < dependency_set.len() {
            let tmp: &Vec<Dependency> = &*self.dependencies.get(&dependency_set[counter]).unwrap();

            for tmp_dependency in tmp {
                let first_dependency = tmp_dependency.first().unwrap();
                if !dependencies_visited.contains(&first_dependency.package_num) {
                    dependency_set.push(first_dependency.package_num);
                    dependencies_visited.insert(first_dependency.package_num);
                }
            }
            counter = counter + 1;
        }

        return dependency_set;
    }

    /// Computes a set of packages that need to be installed to satisfy package_name's deps given the current installed packages.
    /// When a dependency A | B | C is unsatisfied, there are two possible cases:
    ///   (1) there are no versions of A, B, or C installed; pick the alternative with the highest version number (yes, compare apples and oranges).
    ///   (2) at least one of A, B, or C is installed (say A, B), but with the wrong version; of the installed packages (A, B), pick the one with the highest version number.
    pub fn compute_how_to_install(&self, package_name: &str) -> Vec<i32> {
        if !self.package_exists(package_name) {
            return vec![];
        }

        let deps : &Vec<Dependency> = &*self.dependencies.get(self.get_package_num(package_name)).unwrap();
        let mut dependencies_to_add : Vec<i32> = vec![];
        let mut dependencies_visited = HashSet::new();
        let mut counter = 0;

        // implement more sophisticated worklist
        for depend in deps {
            if self.dep_is_satisfied(depend).is_none() {
                let wrong_version_deps = self.dep_satisfied_by_wrong_version(depend);

                if wrong_version_deps.len() > 0 {
                    let mut package_number_chosen = 0;
                    let mut max_version = &"0".parse::<debversion::DebianVersionNum>().unwrap();

                    for package_name in wrong_version_deps {
                        // debversion::cmp_debversion_with_op(op, iv, &v)
                        let package_num = self.get_package_num(&package_name);
                        let compared_dev_string = &self.installed_debvers[&package_num];
                        let operation = ">=".parse::<debversion::VersionRelation>().unwrap();

                        if debversion::cmp_debversion_with_op(&operation, &compared_dev_string, &max_version) {
                            package_number_chosen = *package_num;
                            max_version = compared_dev_string;
                        }
                    }

                    dependencies_to_add.push(package_number_chosen);
                    dependencies_visited.insert(package_number_chosen);
                } else {
                    let mut package_number_chosen = 0;
                    let mut max_version = &"0".parse::<debversion::DebianVersionNum>().unwrap();

                    for package_struct in depend {
                        let package_num = package_struct.package_num;
                        if self.available_debvers.contains_key(&package_num) {
                            let compared_dev_string = &self.available_debvers[&package_num];
                            let operation = ">=".parse::<debversion::VersionRelation>().unwrap();

                            if debversion::cmp_debversion_with_op(&operation, &compared_dev_string, &max_version) {
                                package_number_chosen = package_num;
                                max_version = compared_dev_string;
                            }
                        }
                    }

                    dependencies_to_add.push(package_number_chosen);
                    dependencies_visited.insert(package_number_chosen);
                }
            }
        }

        while counter < dependencies_to_add.len() {
            let tmp: &Vec<Dependency> = &*self.dependencies.get(&dependencies_to_add[counter]).unwrap();

            for tmp_dependency in tmp {
                if self.dep_is_satisfied(tmp_dependency).is_none() {
                    let wrong_version_deps = self.dep_satisfied_by_wrong_version(tmp_dependency);

                    if wrong_version_deps.len() > 0 {
                        let mut package_number_chosen = 0;
                        let mut max_version = &"0".parse::<debversion::DebianVersionNum>().unwrap();

                        for package_name in wrong_version_deps {
                            // debversion::cmp_debversion_with_op(op, iv, &v)
                            let package_num = self.get_package_num(&package_name);
                            let compared_dev_string = &self.installed_debvers[&package_num];
                            let operation = ">=".parse::<debversion::VersionRelation>().unwrap();

                            if debversion::cmp_debversion_with_op(&operation, &compared_dev_string, &max_version) {
                                package_number_chosen = *package_num;
                                max_version = compared_dev_string;
                            }
                        }
                        if !dependencies_visited.contains(&package_number_chosen) {
                            dependencies_to_add.push(package_number_chosen);
                            dependencies_visited.insert(package_number_chosen);
                        }
                    } else {
                        let mut package_number_chosen = 0;
                        let mut max_version = &"0".parse::<debversion::DebianVersionNum>().unwrap();

                        for package_struct in tmp_dependency {
                            let package_num = package_struct.package_num;
                            if self.available_debvers.contains_key(&package_num) {
                                let compared_dev_string = &self.available_debvers[&package_num];
                                let operation = ">=".parse::<debversion::VersionRelation>().unwrap();

                                if debversion::cmp_debversion_with_op(&operation, &compared_dev_string, &max_version) {
                                    package_number_chosen = package_num;
                                    max_version = compared_dev_string;
                                }
                            }
                        }
                        if !dependencies_visited.contains(&package_number_chosen) {
                            dependencies_to_add.push(package_number_chosen);
                            dependencies_visited.insert(package_number_chosen);
                        }
                    }
                }
            }
            counter = counter + 1;
        }


        return dependencies_to_add;
    }
}
