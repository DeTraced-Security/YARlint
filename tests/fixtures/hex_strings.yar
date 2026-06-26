rule HexStringCoverage {
    strings:
        // --- plain bytes ---
        $hex_single_byte    = { 4D }
        $hex_multiple_bytes = { 4D 5A 90 00 }
        $hex_lowercase      = { 4d 5a 90 00 }
        $hex_mixed_case     = { 4D 5a 90 00 }
        $hex_no_whitespace  = { 4D5A9000 }            // bytes don't need separators

        // --- wildcards ---
        $hex_full_wildcard        = { 4D ?? 90 }
        $hex_high_nibble_wildcard = { 4D ?A 90 }      // high nibble unknown, low = A
        $hex_low_nibble_wildcard  = { 4D B? 90 }      // high = B, low nibble unknown
        $hex_all_wildcards        = { ?? ?? ?? }
        $hex_nibble_wildcard_run  = { 4D ?A B? ?? 90 }

        // --- jumps ---
        $hex_jump_exact           = { 4D [4] 5A }
        $hex_jump_range           = { 4D [4-6] 5A }
        $hex_jump_unbounded_above = { 4D [4-] 5A }    // at least 4 bytes
        $hex_jump_fully_unbounded = { 4D [-] 5A }     // any number of bytes
        $hex_jump_large_bounds    = { 4D [100-500] 5A }
        $hex_multiple_jumps       = { 4D [2-4] 5A [1] 90 }

        // --- alternation ---
        $hex_alternation_simple        = { 4D ( AA | BB ) 5A }
        $hex_alternation_three_branch   = { 4D ( AA | BB | CC ) 5A }
        $hex_alternation_multi_atom    = { 4D ( AA BB | CC DD EE ) 5A }
        $hex_alternation_leading       = { ( 4D | 5A ) 90 }
        $hex_alternation_trailing      = { 4D ( 90 | 5A ) }
        $hex_alternation_with_wildcard = { 4D ( AA ?? | BB ) 5A }
        $hex_alternation_with_jump     = { 4D ( AA [2-4] BB | CC ) 5A }
        $hex_alternation_nested        = { 4D ( AA ( BB | CC ) | DD ) 5A }

        // --- combined: byte, wildcard, both nibble wildcards, jump,
        // alternation-with-multi-atom-branch-containing-its-own-jump,
        // and a trailing jump, all in one string ---
        $hex_kitchen_sink = { 4D 5A ?? ?A B? [2-4] ( 90 CC | DD EE [1-2] ) [1] FF }

    condition:
        all of them
}