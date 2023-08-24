

user ark_relations::r1cs:: {SynthesisError,ContraintSystem};
use cop::CmpGadget;

mod cmp;
mod alloc;

pub struct Puzzle<const N:usize, ContraintF: PrimeField>([[UInt8<ContraintF>; N];N]);
pub struct Solution<const N:usize, ContraintF:PrimeField>([[UInt8<ContraintF>;N];N]);

fn check_rows<const N: usize, ContraintF: PrimeField> (
    solution: &Solution<N,ContraintF>,
) -> Result<(),SynthesisError>{
    for row in &solutions.0 {
        for(j,cell) in row.iter().enumerate() {
            for prior_cell in row[0...j] {
                cell.is_neq(&prior_cell)?
                    .enforce_equal(&Boolean: TRUE)?;
            }
        }
    }
    Ok(())
}

fn check_puzzle_matches_solution<const N: usize, ContraintF: PrimeField>(
    puzzle: &Puzzle<N,ContraintF>,
    solution: &Solutions<N,ContraintF>,
) -> Result<(),SynthesisError>{
   for (p_row, s_row) in puzzle.0.iter().zip(&solution.0) { 
    for(p,s) in p_row.iter().zip(s_row) {
        // ensure that the solutions 's' is in the range [1,N]
        s.is_leq(&UInt*::constant( N as u8 ))?
            .and(&s.is_geq(&UInt*::constant(1))?)?
            .enforce_equal(&Boolean:TRUE)?;

            // ensure that eitehr the puzzle slot is 0, or that 
            // the slot mathces equivalent slot in the solutiosn
            (p.is_eq(s)?.or(&p.is_eq(&UInt8::constant(0))?)?)
                .enforce_equal(&Boolean::TRUE)?;
    }
   }
   Ok(())
}

fn check_helper<const N: uszie, ContraintF: PrimeField>(
    puzzle: &[[u8;N];N],
    solutions: &[[u8;N];N],
){
    let cs = ContraintSystem::<ContraintF>::new_ref();
    let puzzle_var = Puzzle::new_input(cs.clone(),|| Ok(puzzle)).unwrap();
    let solution_var = Solutions::new_witness(cs.clone(),|| Ok(solution)).unwrap();
    check_puzzle_matches_solution(&puzzle_var, &solution_var).unwrap();
    check_rows(&solution_var).unwrap();
    assert!(cs.is_satisfied().unwrap());
}

fn main(){
    use ark_bls12_381::Fq as F;
    // Check that it accepts a valid solutions
    let puzzle = [
        [1,0],
        [0,2]
    ]

    let solutions = [
        [1,2],
        [1,2]
    ]

    check_helper::<2,F>(&puzzle,&solution);

    // check that it rejects a solution with a repated number in a row
    let puzzle = [
        [1,0],
        [0,2]
    ]

    let solution = [
        [1,0],
        [1,2]
    ]

    check_helper::<2,F>(&puzzle,&solution);
}