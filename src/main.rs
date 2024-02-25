use pyo3::prelude::*;
use rand::Rng;
use text_io::read;
fn main() -> PyResult<()> {
    let mut len = 10_000_0;
    let mut energy = 1;
    let mult = 1000;
    let mut iterations = len * mult;
    println!(
            "The default values are. Cells:{} ,Energy:{}, Iterations:{}",
            len, energy, iterations
        );
                println!("Enter 0 to start. \n Enter 1 to change number of cells. \n Enter 2 to change number of iterations. \n Enter 3 to change initial Energy.");

    loop {
        let mut opt:String;

        opt=read!();
        
        if opt == "1" {println!("Enter number of cells");
        opt=read!();
            len = opt.parse().expect("Not a valid number");

        } else if opt == "2" {println!("Enter number of iterations");
        opt=read!();
            iterations = opt.parse().expect("Not a valid number");
                    

        } else if opt == "3" {println!("Enter initial Energy");
        opt=read!();
            energy = opt.parse().expect("Not a valid number");
        } else {
            println!("The program is running now. Please wait");
            break;
        }
                println!(
            "The values are now . Cells:{} ,Energy:{}, Iterations:{}",
            len, energy, iterations
        );println!("Enter 0 to start. Enter other numbers to change other values");
    }
    let mut x = rand::thread_rng();

    let mut vs = vec![energy; len];
    let mut score = vec![0; len + 1];
    //only 1000 mult you get a peak in the middle . 10_000 gives exponential
let counter=iterations/100;
    for i in 0..iterations {
        let p = x.gen_range(0..len);
        let q = x.gen_range(0..len);
        if p != q && vs[q] > 0 {
            vs[q] = vs[q] - 1;
            vs[p] = vs[p] + 1;
        }
        if i%counter==0
        {
            println!("We have completed {}% of the iterations",i/counter);
        }
    }

    for i in vs {
        score[i] = score[i] + 1;
    }
    let mut end = 0;
    let mut ko = true;
    for i in (0..=len).rev() {
        if score[i] > 0 {
            if ko {
                end = i;
                ko = false;
            }
            println!("We have cells {} with the energy {}",score[i],i);
        }
    }
    let mut y = "[".to_string();

    for i in 0..end {
        y.push_str(&(score[i] as f64 ).to_string());
        y.push(',');
    }
    y.pop();
    y.push(']');

    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let try_running=py.run(
                        &format!(
"import numpy as np
import matplotlib.pyplot as plt
y={}
x=np.linspace(0,len(y),len(y))
plt.xlabel(\"Energy \\n The initial energy was {}. The number of cells was {}. The number of iterations was {}. \")
plt.ylabel('Cells with energy')
plt.plot(x,y)
plt.show()",
                y,
                energy.to_string(),
                len.to_string(),
                iterations.to_string(),
            ),
            None,
            None,
        );
        match try_running {
            Ok(_) => {}
            Err(_) => {
               
            }
        }
        Ok(())
    })
//This is a slightly janky way to use python code but it is far more readable than the intended way to pass arrays to python.
//It somehow even ends up being slightly faster than passing arrays by calling a function with call1
}
