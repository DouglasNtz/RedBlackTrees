#[derive(Debug)]
enum Color {
    Red,
    Black
}

use std::fmt::{Debug, Display};
use Color::*;

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

    // Quando temos chaves iguais, As rotações mantém a ordem em sucessor, predecessor e inorder.
    // Ou seja, mesmo que a segunda chave igual inserida não seja o filho direito como no caso
    // da arvore binária, ela será o pai e a primeira chave será o filho esquerdo.

    fn left_rotate(self: &mut Self, index: usize) -> bool {

        let menor = index;

        let maior = match self.array[menor].right {
            Some(i) => i,
            None => return false
        };

        let beta = self.array[maior].left;

        match self.array[menor].parent {  // relação entre maior e pai do menor
            Some(i) => {
                if self.array[menor].value < self.array[i].value {
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
        true
    }

    fn right_rotate(self: &mut Self, index: usize) -> bool {

        let maior = index;

        let menor = match self.array[maior].left {
            Some(i) => i,
            None => return false
        };

        let beta = self.array[menor].right;

        match self.array[maior].parent {  // relação entre menor e pai do maior
            Some(i) => {
                if self.array[maior].value < self.array[i].value {
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
        true
    }

    pub fn insert(self: &mut Self, value: T, satelite: E) {

        let vlen = self.array.len();

        match self.root {
            None => {
                let element = Element {value, color: Red, parent: None, left: None, right: None, satelite};
                self.root = Some(0);
                self.array.push(element);
            }
            Some(mut i) => {
                loop {
                    if value < self.array[i].value {
                        match self.array[i].left {
                            Some(j) => {
                                i = j;
                                continue;
                            }
                            None => {
                                let element = Element {value, color: Red, parent: Some(i), left: None, right: None, satelite};
                                self.array.push(element);
                                self.array[i].left = Some(vlen);
                                break;
                            }
                        }
                    } else {
                        match self.array[i].right {
                            Some(j) => {
                                i = j;
                                continue;
                            }
                            None => {
                                let element = Element {value, color: Red, parent: Some(i), left: None, right: None, satelite};
                                self.array.push(element);
                                self.array[i].right = Some(vlen);
                                break;
                            }
                        }
                    }
                }
            }
        }
        self.fixed_up(vlen);
    }

    fn fixed_up(self: &mut Self, mut index: usize) {

        let mut pai = match self.array[index].parent {
            Some(i) => i,
            None => {
                self.array[index].color = Black;
                return;
            }
        };

        let mut avo = match self.array[pai].parent {
            Some(i) => i,
            None => return
        };

        while let Red = self.array[pai].color {

            if self.array[pai].value < self.array[avo].value {
                
                match self.array[avo].right {

                    Some(tio) => {

                        if let Red = self.array[tio].color {
                            self.array[pai].color = Black;
                            self.array[tio].color = Black;
                            self.array[avo].color = Red;

                            index = avo;

                            pai = match self.array[index].parent {
                                Some(i) => i,
                                None => {
                                    self.array[index].color = Black;
                                    return;
                                }
                            };

                            avo = match self.array[pai].parent {
                                Some(i) => i,
                                None => return
                            };
                        } else {

                            if self.array[index].value >= self.array[pai].value {
                                self.left_rotate(pai);
                                index = pai;  // pois o pai virou filho esquerdo do seu filho direito
                                pai = self.array[index].parent.unwrap();
                                avo = self.array[pai].parent.unwrap();
                            }
                            self.array[pai].color = Black;

                            self.array[avo].color = Red;

                            self.right_rotate(avo);
                        }
                    }
                    None => {
                        if self.array[index].value >= self.array[pai].value {
                            self.left_rotate(pai);
                            index = pai;  // pois o pai virou filho esquerdo do seu filho direito
                            pai = self.array[index].parent.unwrap();
                            avo = self.array[pai].parent.unwrap();
                        }
                        self.array[pai].color = Black;

                        self.array[avo].color = Red;

                        self.right_rotate(avo);
                    }
                }
            } else {

                match self.array[avo].left {

                    Some(tio) => {

                        if let Red = self.array[tio].color {
                            self.array[pai].color = Black;
                            self.array[tio].color = Black;
                            self.array[avo].color = Red;

                            index = avo;

                            pai = match self.array[index].parent {
                                Some(i) => i,
                                None => {
                                    self.array[index].color = Black;
                                    return;
                                }
                            };

                            avo = match self.array[pai].parent {
                                Some(i) => i,
                                None => return
                            };

                        } else {

                            if self.array[index].value < self.array[pai].value {
                                self.right_rotate(pai);
                                index = pai;  // pois o pai virou filho direito do seu filho esquerdo
                                pai = self.array[index].parent.unwrap();
                                avo = self.array[pai].parent.unwrap();
                            }

                            self.array[pai].color = Black;

                            self.array[avo].color = Red;

                            self.left_rotate(avo);
                        }
                    }
                    None => {
                        if self.array[index].value < self.array[pai].value {
                            self.right_rotate(pai);
                            index = pai;  // pois o pai virou filho direito do seu filho esquerdo
                            pai = self.array[index].parent.unwrap();
                            avo = self.array[pai].parent.unwrap();
                        }

                        self.array[pai].color = Black;

                        self.array[avo].color = Red;

                        self.left_rotate(avo);
                    }
                }
            }
        }

        self.array[self.root.unwrap()].color = Black;
    }

    pub fn minimum(self: &Self) -> Option<&T> {
        match self.root {
            None => {
                println!("Árvore não contém elementos.");
                None
            }
            Some(mut i) => {
                loop {
                    match self.array[i].left {
                        Some(j) => {
                            i = j;
                            continue;
                        }
                        None => {
                            break Some(&self.array[i].value)
                        }
                    }
                }
            }
        }
    }

    pub fn maximum(self: &Self) -> Option<&T> {
        match self.root {
            None => {
                println!("Árvore não contém elementos.");
                None
            }
            Some(mut i) => {
                loop {
                    match self.array[i].right {
                        Some(j) => {
                            i = j;
                            continue;
                        }
                        None => {
                            break Some(&self.array[i].value)
                        }
                    }
                }
            }
        }
    }

    pub fn get(self: &Self, value: &T) -> Option<&E> {  // retorna a primeira chave igual a value encontrada

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
                        break Some(&self.array[i].satelite)
                    }
                }
            }
        }
    }

    pub fn get_mut(self: &mut Self, value: &T) -> Option<&mut E> {  // retorna a primeira chave igual a value encontrada

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
                        break Some(&mut self.array[i].satelite)
                    }
                }
            }
        }
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


    pub fn inorder(self: &Self) -> Vec<(&T,&E)> {  // estável de acordo com a ordem de insert (ou seja, por altura na árvore)

        let mut w = vec![];

        match self.root {
            Some(i) => {
                self.inorder_aux(i)
            }
            None => w
        }
    }

    fn inorder_aux(self: &Self, index: usize) -> Vec<(&T,&E)> {

        let mut w = vec![];

        match self.array[index].left {
            Some(i) => {
                let mut wl = self.inorder_aux(i);
                w.append(&mut wl);
            }
            None => {}
        }

        w.push((&self.array[index].value, &self.array[index].satelite));

        match self.array[index].right {
            Some(i) => {
                let mut wr = self.inorder_aux(i);
                w.append(&mut wr);
            }
            None => {}
        }
        w
    }

    pub fn len(self: &Self) -> usize {

        self.array.len()
    }

    fn minimum_aux(self: &Self, mut index: usize) -> usize {

        loop {
            match self.array[index].left {
                Some(j) => {
                    index = j;
                    continue;
                }
                None => {
                    break index
                }
            }
        }
    }

    fn maximum_aux(self: &Self, mut index: usize) -> usize {

        loop {
            match self.array[index].right {
                Some(j) => {
                    index = j;
                    continue;
                }
                None => {
                    break index
                }
            }
        }
    }


    fn sucessor(self: &Self, index: usize) -> Option<usize> {

        match self.array[index].right {

            Some(i) => Some(self.minimum_aux(i)),

            None => {
                let mut pai = self.array[index].parent;
                loop {
                    match pai {
                        Some(j) => {
                            if self.array[j].value > self.array[index].value {   // se for igual não é sucessor, é predecessor
                                return Some(j)                              // o primeiro maior nessa busca éo sucessor
                            } else {
                                pai = self.array[j].parent;
                                continue;
                            }
                        },
                        None => return None
                    }
                }
            }
        }
    }

    fn predecessor(self: &Self, index: usize) -> Option<usize> {

        match self.array[index].left {

            Some(i) => Some(self.maximum_aux(i)),

            None => {
                let mut pai = self.array[index].parent;
                loop {
                    match pai {
                        Some(j) => {
                            if self.array[j].value <= self.array[index].value {
                                return Some(j)
                            } else {
                                pai = self.array[j].parent;
                                continue;
                            }
                        },
                        None => return None
                    }
                }
            }
        }
    }

    pub fn get_sucessor(self: &Self, value: &T) -> Option<(&T, &E)> {

        let opt_index = self.get_index(value);

        match opt_index {

            Some(index) => {
                match self.sucessor(index) {
                    Some(i) => Some((&self.array[i].value, &self.array[i].satelite)),
                    None => None
                }
            }
            None => {
                println!("Valor procurado não existe na árvore.");
                None
            }
        }


    }

    pub fn get_predecessor(self: &Self, value: &T) -> Option<(&T, &E)> {

        let opt_index = self.get_index(value);

        match opt_index {

            Some(index) => {
                match self.predecessor(index) {
                    Some(i) => Some((&self.array[i].value, &self.array[i].satelite)),
                    None => None
                }
            }
            None => {
                println!("Valor procurado não existe na árvore.");
                None
            }
        }
    }

    pub fn counting_blacks(self: &Self) -> (bool, Option<usize>) {

        let root = match self.root {
            Some(k) => k,
            None => return (true, Some(0))
        };

        let mut folhas = vec![];

        for (index, element) in self.array.iter().enumerate() {
            match (&element.left, &element.right) {
                (None, None) => folhas.push(index),
                _ => {}
            }
        }
        let mut counting: Vec<usize> = vec![0;folhas.len()];

        for (i, index) in folhas.iter().enumerate() {

            let mut j = *index;

            loop {
                counting[i] += if let Black = self.array[j].color { 1 } else {0};

                if j != root {
                    j = self.array[j].parent.unwrap();
                    continue;
                } else {
                    break;
                }
            }
        }

        if counting.iter().min().unwrap() == counting.iter().max().unwrap() {
            (true, Some(counting[0]))
        } else {
            (false, None)
        }
    }

    pub fn red_not_parent_red(self: &Self) -> bool {

        let root = match self.root {
            Some(k) => k,
            None => return true
        };

        let mut folhas = vec![];

        for (index, element) in self.array.iter().enumerate() {
            match (&element.left, &element.right) {
                (None, None) => folhas.push(index),
                _ => {}
            }
        }

        for index in folhas {

            let mut j = index;

            while j != root {

                if let (&Red, &Red) = (&self.array[j].color, &self.array[self.array[j].parent.unwrap()].color) {
                    return false
                } else {
                    j = self.array[j].parent.unwrap();
                    continue;
                }
            }
        }
        true
    }

    pub fn root_is_black(self: &Self) -> bool {

        match self.root {
            Some(i) => if let Black = self.array[i].color { true } else { false },
            None => true
        }
    }
}

impl<T: Debug + PartialOrd, E> RedBlackTree<T, E>  {
    pub fn print_elements(self: &Self) {

        for (i, element) in self.array.iter().enumerate() {
            println!("Index: {}, Valor: {:?}, Pai: {:?}, Left: {:?}, Right: {:?}, Cor: {:?}",
            i, element.value, element.parent, element.left, element.right, element.color)
        }
    }
}