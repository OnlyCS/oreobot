def main [
	--release
	--skip-prisma (-s)
] {
	let wd = (pwd)

	# Generate prisma client
	if not $skip_prisma {
		if $release {
			cargo prisma-release db push
		} else {
			cargo prisma db push
		}
	}

	# Build
	cd crates

	ls
		| where type == dir
		| where (
			ls $it.name
				| where name =~ "Cargo.toml"
				| first
				| get name
				| open --raw
				| lines
				| where $it == "[lib]"
				| length
		) == 0
		| where $it.name != "prisma-cli"
		| get name
		| each { |dir| 
			cd $dir

			let name = (open "Cargo.toml" | get package.name)

			if $release {
				cargo build --release
				cp ($wd | append "target/release" | append $name | str join "/") .
			} else {
				cargo build
				cp ($wd | append "target/debug" | append $name | str join "/") .
			}

			cd ..
		}

	cd ..
}