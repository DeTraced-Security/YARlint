rule EmptyStringBlock
{

    meta:
        author = "DeTraced Security"
        description = "Test rule for YARlint"
        
    strings:


    condition:
        all of ($s*) or $pdb
}
