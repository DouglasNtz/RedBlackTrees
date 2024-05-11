#[derive(Debug)]
enum Color {
    Red,
    Black
}
#[derive(Debug)]
struct Element<T: PartialOrd, E> {
    value: T,
    color: Color,
    parent: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
    satelite: E
}

#[derive(Debug)]
pub struct RedBlackTree<T: PartialOrd, E> {
    root: Option<usize>,
    array: Vec<Element<T,E>>
}

impl<T: PartialOrd, E> RedBlackTree<T,E> {

    pub fn new() -> Self {

        Self {root: None, array: Vec::new()}
    }

    fn get_index(self: &Self, value: &T) -> Option<usize> {  // retorna a primeira chave igual a value encontrada

        match self.root {
            None => {
                println!("Valor procurado não existe na árvore.");
                None
            }
            Some(mut i) => {
                loop {
                    if value < &self.array[i].value {
                        match self.array[i].left {
                            Some(j) => {
                                i = j;
                                continue;
                            }
                            None => {
                                println!("Valor procurado não existe na árvore.");
                                break None
                            }
                        }
                    } else if value > &self.array[i].value{
                        match self.array[i].right {
                            Some(j) => {
                                i = j;
                                continue;
                            }
                            None => {
                                println!("Valor procurado não existe na árvore.");
                                break None
                            }
                        }
                    } else {
                        break Some(i)
                    }
                }
            }
        }
    }

    fn left_rotate(self: &mut Self, key: &T) {

        let menor = match self.get_index(key) {
            Some(i) => i,
            None => return
        };

        let maior = match self.array[menor].right {
            Some(i) => i,
            None => {
                println!("Não é possível rotacionar a árvore nesse nó.");
                return;
            }
        };

        let beta = self.array[maior].left;

        match self.array[menor].parent {  // relação entre maior e pai do menor
            Some(i) => {
                if self.array[menor] < self.array[i] {
                    self.array[i].left = Some(maior);
                } else {
                    self.array[i].right = Some(maior);
                }
                self.array[maior].parent = Some(i);
            }
            None => {
                self.root = Some(maior);
                self.array[maior].parent = None
            }
        }

        self.array[menor].parent = Some(maior);
        self.array[maior].left = Some(menor);
        self.array[menor].right = beta;

        match beta {
            Some(i) => self.array[i].parent = Some(menor),
            None => {}
        }
    }

    fn right_rotate(self: &mut Self, key: &T) {

        let maior = match self.get_index(key) {
            Some(i) => i,
            None => return
        };

        let menor = match self.array[maior].left {
            Some(i) => i,
            None => {
                println!("Não é possível rotacionar a árvore nesse nó.");
                return;
            }
        };

        let beta = self.array[menor].right;

        match self.array[maior].parent {  // relação entre menor e pai do maior
            Some(i) => {
                if self.array[maior] < self.array[i] {
                    self.array[i].left = Some(menor);
                } else {
                    self.array[i].right = Some(menor);
                }
                self.array[menor].parent = Some(i);
            }
            None => {
                self.root = Some(menor);
                self.array[menor].parent = None
            }
        }

        self.array[maior].parent = Some(menor);
        self.array[menor].right = Some(maior);
        self.array[maior].left = beta;

        match beta {
            Some(i) => self.array[i].parent = Some(maior),
            None => {}
        }
    }



}