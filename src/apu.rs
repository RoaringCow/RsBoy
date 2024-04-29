pub struct APU {

    // info here https://gbdev.io/pandocs/Audio_Registers.html
    
    // FF26 NR52 Audio master control
    nr52: u8,

    // FF25 nr51 sound panning
    nr51: u8,

    // FF24 nr50 Master volume & VIN panning
    nr50: u8,


    // ------- Sound channel 1 -------
    // FF10 nr10: sweep
    nr10: u8,

    // FF11 nr11: lenght timer & duty cycle
    nr11: u8,

    // FF12 n12: volume & envelope
    n12: u8,

    // FF13 n13: period low (write only)
    n13: u8,

    // FF14 n14: period high & control
    // ------------------------------

}
