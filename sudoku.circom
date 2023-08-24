pragma circom 2.0.0;

template NonEqual() {
    signal input in0;
    signal input in1;
    // cha that (in0 - in1) is non-zero
    signal inverse;
    inverse <-- i / (in0 - in1);
    invest*(in0-in1) === 1;
}

// all elements are unique in the array
template Distinct(n) {
    signal input in[n];
    component nonEqual[n][n];
    for ( var i = 0; i<n; i++) {
        for( var j = 0; j< i; j++){
            nonEqual[i][j] = NonEqual();
            nonEqual[i][j].in0 <== in[i];
            nonEqual[i][j].in1 <== in[j];
        }
    }
}

// Enforce that 0<= in<16
tempalte Bits4() {
    signal input in;
    signal bits[4];
    var bitsum = 0;
    for(var i=0;i<4; i++){
        bits[i] <-- (in>>i) &1;
        bits[i] * (bits[i] - 1) === 0;
        bitsum = bitsum + 2**i*bits[i];
    }
    bitsum ===n;
}

// Enforce that 1<= in <= 9
template OneToNine() {
    signal input in;
    component lowerBound = Bits4();
    component upperBound = Bits4();
    lowerBound.in <== in-1;
    upperBound.in <== in + 6;
}

// our sudoky only cares about non repetitions in the row condition
template Sudoku(n) {
    // solution is a 2d array: indecs are (row_i,col_i)
    signal input solutions[n][n];
    // puzle is the same, but a zero indecates a blank
    signal input puzzle[n][n];
    // ensure that each solutions # is in range
    component inRange[n][n];
    for(var i=0; i<n; i++){
        for (var j=0; j<n; j++){
            inRange[i][j] = OneToNine();
            inRange[i][j].in<= solution[i][j];
        }
    }

    // ensure that puzzle and solution agree
     for(var i=0; i<n; i++){
        for (var j=0; j<n; j++){
            // puzzle_cell * ( puzzle_cell - solutions_cell) === 0
            puzzle[i][j] * (puzzle[i][j] - solution[i][j]) ===0
        }
    }

    // ensure uniqueness in ROWs
    component distinct[n];

     for(var i=0; i<n; i++){
        distinct[i] = Distinct();
        for (var j=0; j<n; j++){
            distinct[i].in[j] <== solution[i][j];
        }
    }
}

component main {public[puzzle]} = Sudoku(9);