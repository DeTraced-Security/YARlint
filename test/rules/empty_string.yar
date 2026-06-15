rule EmptyString {

    meta:
        author = "DeTraced Security"
        description = "test rule for YARlint"

    strings:
        $s1 = ""
        $s2 = "test"
    
    condition:
        all of ($s*)
}