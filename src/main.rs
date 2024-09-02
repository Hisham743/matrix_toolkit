use cliclack::log;
use matrix_core::{Matrix, MatrixError};
use std::{cell::RefCell, collections::HashMap, io, ops::Deref, process, rc::Rc};

fn main() -> io::Result<()> {
    let cli = Cli {
        matrices: RefCell::new(HashMap::new()),
    };

    cli.start()?;
    Ok(())
}

struct Cli {
    matrices: RefCell<HashMap<String, Rc<Matrix>>>,
}

impl Cli {
    fn start(&self) -> io::Result<()> {
        cliclack::clear_screen()?;
        cliclack::intro("Matrix Toolkit")?;

        self.main_menu()?;
        Ok(())
    }

    fn main_menu(&self) -> io::Result<()> {
        let action = cliclack::select("What do you want to do?")
            .item("create", "Create a Matrix", "")
            .item("operate", "Perform Matrix Operations", "")
            .item("properties", "Check Matrix Properties", "")
            .item("exit", "Exit", "")
            .interact()?;

        match action {
            "create" => self.create_matrix()?,
            "operate" => self.perform_operations()?,
            "properties" => self.check_properties()?,
            "exit" => process::exit(0),
            _ => unreachable!(),
        }

        Ok(())
    }

    fn prompt_name(&self) -> io::Result<String> {
        let mut name: String = cliclack::input("Name of the matrix")
            .validate_interactively(|input: &String| {
                if input.starts_with(|charecter: char| charecter.is_numeric()) {
                    return Err("Names cannot start with a number");
                }

                if input
                    .chars()
                    .any(|charecter: char| charecter.is_whitespace())
                {
                    return Err("Names cannot have whitespaces in them");
                }

                Ok(())
            })
            .interact()?;

        while self.matrices.borrow().contains_key(&name) {
            log::error("A matrix with that name is already created")?;
            name = cliclack::input("Choose a new name").interact()?;
        }
        Ok(name)
    }

    fn prompt_size(&self, prompt: &'static str, err_msg: &'static str) -> io::Result<usize> {
        let size = cliclack::input(prompt)
            .validate_interactively(move |input: &String| {
                if !input.parse::<usize>().is_ok_and(|num| num > 0) {
                    Err(err_msg)
                } else {
                    Ok(())
                }
            })
            .interact()?;

        Ok(size)
    }

    fn create_matrix(&self) -> io::Result<()> {
        let creation_method = cliclack::select("How do you want to create the matrix?")
            .item("manual", "Enter matrix elements manually", "")
            .item("zero", "Create a zero matrix", "")
            .item("identity", "Create an identity matrix", "")
            .item("scalar", "Create a scalar matrix", "")
            .item("diagonal", "Create a diagonal matrix", "")
            .item("back", "Back to main menu", "")
            .interact()?;

        match creation_method {
            "manual" => {
                let matrix_name = self.prompt_name()?;
                let input_string: String = cliclack::input("Enter the elements")
                    .multiline()
                    .validate_interactively(|input: &String| {
                        if input.lines().any(|line| {
                            line.split_whitespace()
                                .any(|value| value.parse::<f64>().is_err())
                        }) {
                            return Err(
                                "It should be real numbers seperated by a whitespace on each line",
                            );
                        }

                        let mut lines = input.lines();
                        let first_row_length =
                            lines.next().unwrap_or("").split_whitespace().count();

                        if !lines.all(|line| line.split_whitespace().count() == first_row_length) {
                            return Err("Every row should have the same number of elements");
                        }

                        Ok(())
                    })
                    .interact()?;

                let values = input_string
                    .lines()
                    .map(|line| {
                        line.split_whitespace()
                            .map(|num| num.parse().unwrap())
                            .collect()
                    })
                    .collect();

                let matrix: Matrix = Matrix::new_with_data(values).unwrap();
                cliclack::note(&matrix_name, &matrix)?;
                self.matrices
                    .borrow_mut()
                    .insert(matrix_name, Rc::new(matrix));
            }
            "zero" => {
                let matrix_name = self.prompt_name()?;
                let rows = self.prompt_size(
                    "Number of rows",
                    "Number of rows should be a whole number greater than 0",
                )?;
                let columns = self.prompt_size(
                    "Number of columns",
                    "Number of columns should be a whole number greater than 0",
                )?;

                let matrix = Matrix::new_zero_matrix(rows, columns).unwrap();
                cliclack::note(&matrix_name, &matrix)?;
                self.matrices
                    .borrow_mut()
                    .insert(matrix_name, Rc::new(matrix));
            }
            "identity" => {
                let matrix_name = self.prompt_name()?;
                let size = self.prompt_size(
                    "Size of the matrix",
                    "Size should be a whole number greater than 0",
                )?;

                let matrix = Matrix::nth_identity(size).unwrap();
                cliclack::note(&matrix_name, &matrix)?;
                self.matrices
                    .borrow_mut()
                    .insert(matrix_name, Rc::new(matrix));
            }
            "scalar" => {
                let matrix_name = self.prompt_name()?;
                let scalar: f64 = cliclack::input("The scalar value")
                    .validate_interactively(|input: &String| {
                        if !input.parse::<f64>().is_ok() {
                            Err("The scalar should be a real number")
                        } else {
                            Ok(())
                        }
                    })
                    .interact()?;

                let size = self.prompt_size(
                    "Size of the matrix",
                    "Size should be a whole number greater than 0",
                )?;

                let matrix: Matrix = Matrix::new_scalar_matrix(scalar, size).unwrap();
                cliclack::note(&matrix_name, &matrix)?;
                self.matrices
                    .borrow_mut()
                    .insert(matrix_name, Rc::new(matrix));
            }
            "diagonal" => {
                let matrix_name = self.prompt_name()?;
                let input_string: String = cliclack::input("Diagonal elements")
                    .validate_interactively(|input: &String| {
                        if input
                            .split_whitespace()
                            .any(|value| value.parse::<f64>().is_err())
                        {
                            Err("It should be real numbers seperated by a whitespace")
                        } else {
                            Ok(())
                        }
                    })
                    .interact()?;

                let values = input_string
                    .split_whitespace()
                    .map(|num| num.parse().unwrap())
                    .collect();

                let matrix: Matrix = Matrix::new_diagonal_matrix(&values).unwrap();
                cliclack::note(&matrix_name, &matrix)?;
                self.matrices
                    .borrow_mut()
                    .insert(matrix_name, Rc::new(matrix));
            }
            "back" => self.main_menu()?,
            _ => unreachable!(),
        }

        if cliclack::confirm("Do you want to create another matrix?").interact()? {
            self.create_matrix()
        } else {
            self.main_menu()
        }
    }

    fn prompt_matrix(&self, prompt: &'static str) -> io::Result<Rc<Matrix>> {
        let mut matrix_name: String = cliclack::input(prompt).interact()?;
        while !self.matrices.borrow().contains_key(&matrix_name) {
            log::error("A matrix with that name is not created yet")?;
            let create_matrix = cliclack::confirm("Do you want to create a new matrix?")
                .initial_value(true)
                .interact()?;

            if create_matrix {
                self.create_matrix()?;
            } else {
                matrix_name = cliclack::input("Enter the correct name of the matrix").interact()?;
            }
        }

        Ok(Rc::clone(self.matrices.borrow().get(&matrix_name).unwrap()))
    }

    fn perform_operations(&self) -> io::Result<()> {
        let operation = cliclack::select("Choose an operation")
            .item("add", "Addition", "")
            .item("subtract", "Subtraction", "")
            .item("multiply", "Matrix Multiplication", "")
            .item("scale", "Scalar Multiplication", "")
            .item("trace", "Trace", "")
            .item("transpose", "Transpose", "")
            .item("determinant", "Determinant", "")
            .item("adjoint", "Adjoint", "")
            .item("inverse", "Inverse", "")
            .item("back", "Back to main menu", "")
            .interact()?;

        match operation {
            "add" => {
                let matrix1 = self.prompt_matrix("Name of the first matrix")?;
                let matrix2 = self.prompt_matrix("Name of the second matrix")?;
                let result = matrix1.deref() + matrix2.deref();

                match result {
                    Err(_) => log::error("Dimensions of the two matrices do not match")?,
                    Ok(sum) => cliclack::note("Sum", sum)?,
                };
            }
            "subtract" => {
                let matrix1 = self.prompt_matrix("Name of the first matrix")?;
                let matrix2 = self.prompt_matrix("Name of the second matrix")?;
                let result = matrix1.deref() - matrix2.deref();

                match result {
                    Err(_) => log::error("Dimensions of the two matrices do not match")?,
                    Ok(difference) => cliclack::note("Difference", difference)?,
                };
            }
            "multiply" => {
                let matrix1 = self.prompt_matrix("Name of the first matrix")?;
                let matrix2 = self.prompt_matrix("Name of the second matrix")?;
                let result = matrix1.deref() * matrix2.deref();

                match result {
                    Err(_) => log::error("Number of columns of the first matrix is not equal to the number of rows of the second matrix")?,
                    Ok(product) => cliclack::note("Product", product)?,
                };
            }
            "scale" => {
                let scalar: f64 = cliclack::input("The scalar value")
                    .validate_interactively(|input: &String| {
                        if !input.parse::<f64>().is_ok() {
                            Err("The scalar should be a real number")
                        } else {
                            Ok(())
                        }
                    })
                    .interact()?;

                let matrix = self.prompt_matrix("Name of the matrix")?;
                let scaled_matrix = scalar * matrix.deref();
                cliclack::note("Scaled Matrix", scaled_matrix)?;
            }
            "trace" => {
                let matrix = self.prompt_matrix("Name of the matrix")?;
                let result = matrix.trace();

                match result {
                    Err(_) => log::error("Only square matrices have traces")?,
                    Ok(trace) => log::success(format!("Trace = {trace}"))?,
                };
            }
            "transpose" => {
                let matrix = self.prompt_matrix("Name of the matrix")?;
                let transpose = matrix.transpose();
                cliclack::note("Transpose", transpose)?;
            }
            "determinant" => {
                let matrix = self.prompt_matrix("Name of the matrix")?;
                let result = matrix.determinant();

                match result {
                    Err(_) => log::error("Only square matrices have determinants")?,
                    Ok(determinant) => log::success(format!("Determinant = {determinant}"))?,
                };
            }
            "adjoint" => {
                let matrix = self.prompt_matrix("Name of the matrix")?;
                let result = matrix.adjoint();

                match result {
                    Err(_) => log::error("Only square matrices have adjoints")?,
                    Ok(adjoint) => cliclack::note("Adjoint", adjoint)?,
                };
            }
            "inverse" => {
                let matrix = self.prompt_matrix("Name of the matrix")?;
                let result = matrix.inverse();

                match result {
                    Err(err) => match err {
                        MatrixError::NonSquareMatrix => {
                            log::error("Only square matrices have inverses")?
                        }
                        MatrixError::SingularMatrix => {
                            log::error("Singular matrices do not have inverse")?
                        }
                        _ => unreachable!(),
                    },
                    Ok(inverse) => cliclack::note("Inverse", inverse)?,
                };
            }
            "back" => self.main_menu()?,
            _ => unreachable!(),
        }

        if cliclack::confirm("Do you want to perform any other matrix operations?").interact()? {
            self.perform_operations()
        } else {
            self.main_menu()
        }
    }

    fn check_properties(&self) -> io::Result<()> {
        let matrix = self.prompt_matrix("Name of the matrix")?;

        let value_labels = [
            ("square", "Is Square"),
            ("symmetric", "Is Symmetric"),
            ("skew_symmetric", "Is Skew Symmetric"),
            ("diagonal", "Is Diagonal"),
            ("scalar", "Is Scalar"),
            ("identity", "Is Identity"),
            ("zero", "Is Zero"),
            ("singular", "Is Singular"),
            ("back", "Back to main menu"),
        ];

        let properties = cliclack::multiselect("What properties do you want to check?")
            .items(
                &value_labels
                    .iter()
                    .map(|(value, label)| (*value, *label, ""))
                    .collect::<Vec<(&str, &str, &str)>>(),
            )
            .interact()?;

        if properties.contains(&"back") {
            self.main_menu()?;
        }

        let mut results = HashMap::new();
        properties.iter().for_each(|property| {
            if let "back" = *property {
                return;
            }

            let result = match *property {
                "square" => matrix.is_square(),
                "symmetric" => matrix.is_symmetric(),
                "skew_symmetric" => matrix.is_skew_symmetric(),
                "diagonal" => matrix.is_diagonal(),
                "scalar" => matrix.is_scalar(),
                "identity" => matrix.is_identity(),
                "zero" => matrix.is_zero(),
                "singular" => matrix.is_singular(),
                _ => unreachable!(),
            };

            let label = value_labels
                .iter()
                .find(|(value, _)| value == property)
                .map(|(_, label)| *label)
                .unwrap();

            results.insert(label, result);
        });

        let mut output = String::new();
        results.iter().for_each(|(label, result)| {
            output.push_str(&format!("{label}: {}\n", if *result { "✅" } else { "❌" }))
        });

        log::success(output)?;

        if cliclack::confirm("Do you want to check properties of any other matrices?").interact()? {
            self.check_properties()
        } else {
            self.main_menu()
        }
    }
}
