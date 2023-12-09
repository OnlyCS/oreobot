#!/usr/bin/nu

def main [
	--release
	--skip-prisma (-n)
	--stop (-s)
] {
	if $stop {
		docker compose down
		return
	}

	let owd = (pwd)

	if not $skip_prisma {
		if $release {
			cargo prisma-release db push
		} else {
			cargo prisma db push
		}
	}

	cd crates

	mut tmp_files = []

	let dirs = ls
		| where type == dir 
		| where (
			ls $it.name 
			| where name =~ Cargo.toml 
			| first 
			| get name 
			| open --raw 
			| lines 
			| where $it == "[lib]" 
			| length
		) == 0
		| get name
	
	for dir in $dirs {
		cd $dir
		let name = (open "Cargo.toml" | get package.name)

		
		if $release {
			cargo build --release
			cp ($owd | append "target/release" | append $name | str join "/") .
		} else {
			cargo build
			cp ($owd | append "target/debug" | append $name | str join "/") .
		}

		$tmp_files = $tmp_files | append $name
		cd ..
	}
	
	cd ..

	docker compose up -d --build

	$tmp_files | each { |f| 
		rm $f	
	}
}
