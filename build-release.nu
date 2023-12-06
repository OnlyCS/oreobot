#!/usr/bin/nu

cd crates

ls | where type == dir | where (ls $it.name | where name =~ "build-release.nu" | length) > 0 | each { |it| 
	cd $it.name
	nu ./build-release.nu
	cd ..
}

cd ..