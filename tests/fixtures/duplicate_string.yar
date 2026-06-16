rule DuplicateString {

    meta:
        author = "DeTraced Security"
        description = "test rule for YARlint"
        
    strings:
        $s1 = "test" wide ascii
        $s2 = "test" wide ascii


    condition:
        all of ($s*)
}
