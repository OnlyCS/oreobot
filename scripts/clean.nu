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
	| get name
	| each { |dir| 
		cd $dir
		
		let name = (open "Cargo.toml" | get package.name)
		rm $name
		
		cd ..
	}