rule BadRule
{

    meta:
        author = "AlienVault Labs"
        info = "CommentCrew-threat-apt1"
        
    strings:
        $s1 = "Kill process success!" wide ascii
        $s2 = "Kill process failed!" wide ascii
        $s3 = ""
        $s4 = "" wide ascii
        $pdb = "glooxtest.pdb" wide ascii
        $s5 = "C:/Windows/System32"

    condition:
        all of ($s*) or $pdb
}
