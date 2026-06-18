rule MissingMeta {

    meta:
        author = "DeTraced Security"

    strings:
        $s1 = "test1"
        $s2 = "test2"
        
    condition:
        all of ($s*)
}