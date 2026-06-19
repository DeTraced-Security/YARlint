rule VariousStrings {

    meta:
        author = "AlienVault Labs"
        description = "test rule for yarlint"
        reference = "https://detraced.org"
        date = "2026-06-18"
        
    strings:
        $string = "mal.exe" wide ascii
        $hex = { a6 f8 ?d 82 }
        $regex = /md5: [0-9a-fA-F]{32}/

    condition:
        all of them
}