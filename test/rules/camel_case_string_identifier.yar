rule CamelCaseRuleIdentifier {

    meta:
        author = "DeTraced Security"
        description = "test rule for YARlint"

    strings:
        $stringOne = "test1"
        $stringTwo = "test2"
        
    condition:
        all of ($s*)
}