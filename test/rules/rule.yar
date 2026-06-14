rule GoodRule
{

    meta:
        author = "AlienVault Labs"
        info = "CommentCrew-threat-apt1"
        
    strings:
        $s1 = "Kill process success!" wide ascii
        $s2 = "Kill process failed!" wide ascii
        $s3 = "Sleep success!" wide ascii
        $s4 = "based on gloox" wide ascii
        $pdb = "glooxtest.pdb" wide ascii
        $s5 = "C:/Windows/System32"

    condition:
        all of ($s*) or $pdb
}
