rule Bad-Rule-Name {

    meta:
        author = "DeTraced Security"
        description = "test rule for YARlint"

    strings:
        $s1 = "test1"
        $s2 = "test2"
        
    condition:
        all of ($s*)
}