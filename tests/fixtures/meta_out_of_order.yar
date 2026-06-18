rule MetaOutOfOrder {

    meta:
        reference = "example.com"
        description = "test rule for YARlint"
        date = "2026-06-18"
        author = "DeTraced Security"

    strings:
        $s1 = "test1"
        $s2 = "test2"
        
    condition:
        all of ($s*)
}