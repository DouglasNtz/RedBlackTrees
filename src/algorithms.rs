#[derive(Debug, Copy, Clone)]
enum Color {
    Red,
    Black
}

#[derive(Debug, Copy, Clone)]
enum Child {
    Left,
    Right
}

use std::fmt::{Debug, Display};
use Color::*;
use Child::*;

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

#[derive(Debug)]
pub struct RedBlackTreeWithReps<T: PartialOrd, E> {
    root: Option<usize>,
    array: Vec<Element<T,E>>
}

impl<T: PartialOrd + Debug, E: Debug> RedBlackTree<T,E> {

    pub fn new() -> Self {

        Self {root: None, array: Vec::new()}
    }

    fn is_root(self: &Self, index: usize) -> bool {

        match self.root {
            Some(i) => {
                if i == index { true } else { false }
            },
            None => false
        }
    }

    fn is_left(self: &Self, index: usize) -> bool {

        match self.array[index].parent {
            Some(i) => {
                match self.array[i].left {
                    Some(j) => {
                        if j == index { true } else { false }
                    }
                    None => false
                }
            },
            None => false
        }
    }

    fn is_right(self: &Self, index: usize) -> bool {

        match self.array[index].parent {
            Some(i) => {
                match self.array[i].right {
                    Some(j) => {
                        if j == index { true } else { false }
                    }
                    None => false
                }
            },
            None => false
        }
    }

    fn left_is_nil(self: &Self, pai: usize) -> bool {

        match self.array[pai].left {
            Some(i) => false,
            None => true
        }
    }

    fn right_is_nil(self: &Self, pai: usize) -> bool {

        match self.array[pai].right {
            Some(i) => false,
            None => true
        }
    }

    fn left_rotate(self: &mut Self, index: usize) -> bool {

        let menor = index;

        let maior = match self.array[menor].right {
            Some(i) => i,
            None => return false
        };

        let beta = self.array[maior].left;

        match self.array[menor].parent {  // relação entre maior e pai do menor
            Some(i) => {
                if self.is_left(menor) {
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
                if self.is_left(maior) {
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
                    } else if value > self.array[i].value {
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
                    } else {
                        return;  // não insere chave repetida. Aborta inserção.
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

            if self.is_left(pai) {

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
                            if self.is_right(index) {
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
                        if self.is_right(index) {
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
                            if self.is_left(index) {
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
                        if self.is_left(index) {
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

    pub fn deletion(self: &mut Self, value: &T) -> Option<E> {

        let opt_index = self.get_index(value);

        let index = match opt_index {
            Some(i) => i,
            None => return None
        };

        let mut fixed = if let Red = self.array[index].color { false } else { true };

        let mut x = Left;
        let mut xp = 0;

        match (self.array[index].left, self.array[index].right) {
            (None, None) => {
                match self.array[index].parent {
                    Some(i) => {
                        if self.is_left(index) {
                            self.array[i].left = None;
                            (x, xp) = (Left, i);
                        } else {
                            self.array[i].right = None;
                            (x, xp) = (Right, i);
                        }
                    },
                    None => {
                        self.root = None;
                        fixed = false;
                    }
                }
            },
            (None, Some(j2)) => {
                match self.array[index].parent {
                    Some(i) => {
                        if self.is_left(index) {
                            self.array[i].left = Some(j2);
                            if fixed {
                                self.array[j2].color = Black;
                                fixed = false;
                            }
                        } else {
                            self.array[i].right = Some(j2);
                            if fixed {
                                self.array[j2].color = Black;
                                fixed = false;
                            }
                        }
                        self.array[j2].parent = Some(i);
                    },
                    None => {
                        self.root = Some(j2);
                        self.array[j2].parent = None;
                        self.array[j2].color = Black;
                        fixed = false;
                    }
                }
            },
            (Some(j1), None) => {
                match self.array[index].parent {
                    Some(i) => {
                        if self.is_left(index) {
                            self.array[i].left = Some(j1);
                            if fixed {
                                self.array[j1].color = Black;
                                fixed = false;
                            }
                        } else {
                            self.array[i].right = Some(j1);
                            if fixed {
                                self.array[j1].color = Black;
                                fixed = false;
                            }
                        }
                        self.array[j1].parent = Some(i);
                    },
                    None => {
                        self.root = Some(j1);
                        self.array[j1].parent = None;
                        self.array[j1].color = Black;
                        fixed = false;
                    }
                }
            },
            (Some(j1), Some(j2)) => {

                let sucessor = self.sucessor(index).unwrap();

                let filho_sucessor = self.array[sucessor].right;

                (x, xp) = (Right, sucessor);

                fixed = if let Red = self.array[sucessor].color { false } else { true };

                self.array[sucessor].color = self.array[index].color;

                self.array[j1].parent = Some(sucessor);
                self.array[sucessor].left = Some(j1);

                let pai_sucessor = self.array[sucessor].parent.unwrap();

                match self.array[index].parent {
                    Some(i) => {
                        if self.is_left(index) {
                            self.array[i].left = Some(sucessor);
                        } else {
                            self.array[i].right = Some(sucessor);
                        }
                        self.array[sucessor].parent = Some(i);
                    },
                    None => {
                        self.root = Some(sucessor);
                        self.array[sucessor].parent = None;
                    }
                }

                if sucessor != j2 {

                    (x, xp) = (Left, pai_sucessor);

                    self.array[j2].parent = Some(sucessor);

                    self.array[sucessor].right = Some(j2);

                    match filho_sucessor {
                        Some(k) => {
                            self.array[pai_sucessor].left = Some(k);
                            self.array[k].parent = Some(pai_sucessor);
                        }
                        None => {
                            self.array[pai_sucessor].left = None;
                        }
                    }
                }
            }
        }

        let last = self.len() - 1;
        if index != last {
            match self.array[last].parent {
                Some(i) => {
                    if self.is_left(last) {
                        self.array[i].left = Some(index);
                    } else {
                        self.array[i].right = Some(index);
                    }
                },
                None => {
                    self.root = Some(index);
                }
            }
            match self.array[last].left {
                Some(i) => {
                    self.array[i].parent = Some(index)
                },
                None => {}
            }
            match self.array[last].right {
                Some(i) => {
                    self.array[i].parent = Some(index)
                },
                None => {}
            }
            self.array.swap(index, last);
        }

        if fixed {
            self.deletion_fixed_up(x, xp);
        }

        return Some(self.array.pop().unwrap().satelite)
    }

    fn is_black(self: &Self, x: Child, pai: usize) -> bool {

        match x  {
            Left => {
                match self.array[pai].left {
                    Some(i) => {
                        match self.array[i].color {
                            Red => false,
                            Black => true
                        }
                    }
                    None => true
                }
            }
            Right => {
                match self.array[pai].right {
                    Some(i) => {
                        match self.array[i].color {
                            Red => false,
                            Black => true
                        }
                    }
                    None => true
                }
            }
        }
    }

    fn deletion_fixed_up(self: &mut Self, mut x: Child, mut pai: usize) {

        let root = match self.root {
            Some(i) => i,
            None => return
        };

        let mut irmao;

        let mut condition = self.is_black(x, pai);

        if !condition {
            match x {   // se for vermelho, muda pra preto e nem entra no while
                Left => {
                    let left = self.array[pai].left.unwrap();
                    self.array[left].color = Black;
                }
                Right => {
                    let right = self.array[pai].right.unwrap();
                    self.array[right].color = Black;
                }
            }
        }

        while condition {  // se x se tornar root, colocamos como black e break loop. Senão continuamos.

            if let Left = x {

                irmao = self.array[pai].right.unwrap();  // enquanto index é black e não root, ele tem que ter irmao

                if let Red = self.array[irmao].color {  // logo ele tem dois filhos black, pois seu irmao é black
                    self.array[irmao].color = Black;
                    self.array[pai].color = Red;
                    self.left_rotate(pai);
                    irmao = self.array[pai].right.unwrap();  // é o antigo filho left do irmao, que é black
                }

                if self.is_black(Left, irmao) && self.is_black(Right, irmao) {
                    self.array[irmao].color = Red;
                    match self.array[pai].parent {
                        Some(i) => {
                            if self.is_left(pai) {
                                (x, pai) = (Left, i)
                            } else {
                                (x, pai) = (Right, i)
                            }
                            condition = self.is_black(x, pai);
                        }
                        None => {
                            self.array[pai].color = Black;
                            break;
                        }
                    }
                } else {

                    if self.is_black(Right, irmao) {   // logo o esquerdo é vermelho
                        let sobrinho_left = self.array[irmao].left.unwrap();
                        self.array[sobrinho_left].color = Black;
                        self.array[irmao].color = Red;
                        self.right_rotate(irmao);
                        irmao = self.array[pai].right.unwrap();  // é o antigo filho esquerdo de irmao
                    }
                    self.array[irmao].color = self.array[pai].color;
                    self.array[pai].color = Black;
                    // caso tenhamos passado pelo if acima, o filho direito de irmao é o antigo irmao,
                    // caso não tenhamos passado, é um vermelho .. logo não é Nil
                    let sobrinho_right = self.array[irmao].right.unwrap();
                    self.array[sobrinho_right].color = Black;
                    self.left_rotate(pai);
                    self.array[root].color = Black;
                    break;
                }
            } else {
                irmao = self.array[pai].left.unwrap();  // enquanto index é black e não root, ele tem que ter irmao

                if let Red = self.array[irmao].color {  // logo ele tem dois filhos black, pois seu irmao é black
                    self.array[irmao].color = Black;
                    self.array[pai].color = Red;
                    self.right_rotate(pai);
                    irmao = self.array[pai].left.unwrap();  // é o antigo filho rigth do irmao, que é black
                }

                if self.is_black(Left, irmao) && self.is_black(Right, irmao) {
                    self.array[irmao].color = Red;
                    match self.array[pai].parent {
                        Some(i) => {
                            if self.is_left(pai) {
                                (x, pai) = (Left, i)
                            } else {
                                (x, pai) = (Right, i)
                            }
                            condition = self.is_black(x, pai);
                        }
                        None => {
                            self.array[pai].color = Black;
                            break;
                        }
                    }
                } else {

                    if self.is_black(Left, irmao) {   // logo o direito é vermelho
                        let sobrinho_right = self.array[irmao].right.unwrap();
                        self.array[sobrinho_right].color = Black;
                        self.array[irmao].color = Red;
                        self.left_rotate(irmao);
                        irmao = self.array[pai].left.unwrap();  // é o antigo filho direito de irmao
                    }
                    self.array[irmao].color = self.array[pai].color;
                    self.array[pai].color = Black;
                    // caso tenhamos passado pelo if acima, o filho esquerdo de irmao é o antigo irmao,
                    // caso não tenhamos passado, é um vermelho .. logo não é Nil
                    let sobrinho_left = self.array[irmao].left.unwrap();
                    self.array[sobrinho_left].color = Black;
                    self.right_rotate(pai);
                    self.array[root].color = Black;
                    break;
                }
            }
        }
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

    pub fn get(self: &Self, value: &T) -> Option<&E> {

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

    pub fn get_mut(self: &mut Self, value: &T) -> Option<&mut E> {

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

    fn get_index(self: &Self, value: &T) -> Option<usize> {

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


    pub fn inorder(self: &Self) -> Vec<(&T,&E)> {

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
                            if self.array[j].value > self.array[index].value {
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

    fn predecessor(self: &Self, index: usize) -> Option<usize> {

        match self.array[index].left {

            Some(i) => Some(self.maximum_aux(i)),

            None => {
                let mut pai = self.array[index].parent;
                loop {
                    match pai {
                        Some(j) => {
                            if self.array[j].value < self.array[index].value {
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

    fn indexes_subtree(self: &Self, index: usize) -> Vec<usize> {

        let mut w = vec![];

        match self.array[index].left {
            Some(i) => {
                let mut wl = self.indexes_subtree(i);
                w.append(&mut wl);
            }
            None => {}
        }

        w.push(index);

        match self.array[index].right {
            Some(i) => {
                let mut wr = self.indexes_subtree(i);
                w.append(&mut wr);
            }
            None => {}
        }
        w
    }

    fn counting_blacks(self: &Self, root: usize) -> bool {

        let indexes_subtree = self.indexes_subtree(root);

        let mut folhas = vec![];

        for &index in &indexes_subtree {
            match (&self.array[index].left, &self.array[index].right) {
                (None, None) => folhas.push(index),
                (None, Some(_)) => folhas.push(index),
                (Some(_), None) => folhas.push(index),
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

        counting.iter().min().unwrap() == counting.iter().max().unwrap()

    }

    fn red_not_parent_red(self: &Self) -> bool {

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

                if let (Red, Red) = (self.array[j].color, self.array[self.array[j].parent.unwrap()].color) {
                    return false
                } else {
                    j = self.array[j].parent.unwrap();
                    continue;
                }
            }
        }
        true
    }
    fn min_max_subtree_property(self: &Self, index: usize) -> bool {

        let max_left = match self.array[index].left {
            Some(i) => {
                let v = self.indexes_subtree(i);
                let mut max = v[0];
                for j in v {
                    if self.array[j].value > self.array[max].value {
                        max = j;
                    }
                }
                if self.array[max].value < self.array[index].value { true } else { false }
            }
            None => true
        };

        let min_right = match self.array[index].right {
            Some(i) => {
                let v = self.indexes_subtree(i);
                let mut min = v[0];
                for j in v {
                    if self.array[j].value < self.array[min].value {
                        min = j;
                    }
                }
                if self.array[min].value > self.array[index].value { true } else { false }
            }
            None => true
        };

        max_left && min_right

    }

    fn binary_tree_property(self: &Self) -> bool {

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

                let pai = self.array[j].parent.unwrap();

                if self.array[pai].value > self.array[j].value {

                    match self.array[pai].left {
                        Some(k) => {
                            if k != j {
                                return false
                            }
                        }
                        None => return false
                    }
                    j = pai;
                    continue;

                } else if self.array[pai].value < self.array[j].value {

                    match self.array[pai].right {
                        Some(k) => {
                            if k != j {
                                return false
                            }
                        }
                        None => return false
                    }
                    j = pai;
                    continue;

                } else {
                    return false;
                }
            }
        }
        true
    }

    fn root_is_black(self: &Self) -> bool {

        match self.root {
            Some(i) => if let Black = self.array[i].color { true } else { false },
            None => true
        }
    }

    pub fn is_red_black_tree(self: &Self) -> bool {

        if !self.root_is_black() {
            return false
        }

        if !self.red_not_parent_red() {
            return false
        }

        if !self.binary_tree_property() {
            return false
        }

        for index in 0..self.len() {
            if !self.counting_blacks(index) {
                return false
            }
        }

        for index in 0..self.len() {
            if !self.min_max_subtree_property(index) {
                return false
            }
        }
        true
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


impl<T: PartialOrd, E> RedBlackTreeWithReps<T,E> {

    pub fn new() -> Self {

        Self {root: None, array: Vec::new()}
    }

    fn is_left(self: &Self, index: usize) -> bool {

        match self.array[index].parent {
            Some(i) => {
                match self.array[i].left {
                    Some(j) => {
                        if j == index { true } else { false }
                    }
                    None => false
                }
            },
            None => false
        }
    }

    fn is_right(self: &Self, index: usize) -> bool {

        match self.array[index].parent {
            Some(i) => {
                match self.array[i].right {
                    Some(j) => {
                        if j == index { true } else { false }
                    }
                    None => false
                }
            },
            None => false
        }
    }

    fn left_is_nil(self: &Self, pai: usize) -> bool {

        match self.array[pai].left {
            Some(i) => false,
            None => true
        }
    }

    fn right_is_nil(self: &Self, pai: usize) -> bool {

        match self.array[pai].right {
            Some(i) => false,
            None => true
        }
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
                match self.array[i].left {
                    Some(j) => {
                        if j == menor {
                            self.array[i].left = Some(maior);
                        } else {
                            self.array[i].right = Some(maior);
                        }
                        self.array[maior].parent = Some(i);
                    }
                    None => {
                        self.array[i].right = Some(maior);
                        self.array[maior].parent = Some(i);
                    }
                }
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
                match self.array[i].left {
                    Some(j) => {
                        if j == maior {
                            self.array[i].left = Some(menor);
                        } else {
                            self.array[i].right = Some(menor);
                        }
                        self.array[menor].parent = Some(i);
                    }
                    None => {
                        self.array[i].right = Some(menor);
                        self.array[menor].parent = Some(i);
                    }
                }
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

            if self.is_left(pai) {

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
                            if self.is_right(index) {
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
                        if self.is_right(index) {
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
                            if self.is_left(index) {
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
                        if self.is_left(index) {
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

    pub fn deletion(self: &mut Self, value: &T) -> Option<E> {

        let opt_index = self.get_index(value);

        let index = match opt_index {
            Some(i) => i,
            None => return None
        };

        let mut fixed = if let Red = self.array[index].color { false } else { true };

        let mut x = Left;
        let mut xp = 0;

        match (self.array[index].left, self.array[index].right) {
            (None, None) => {
                match self.array[index].parent {
                    Some(i) => {
                        if self.is_left(index) {
                            self.array[i].left = None;
                            (x, xp) = (Left, i);
                        } else {
                            self.array[i].right = None;
                            (x, xp) = (Right, i);
                        }
                    },
                    None => {
                        self.root = None;
                        fixed = false;
                    }
                }
            },
            (None, Some(j2)) => {
                match self.array[index].parent {
                    Some(i) => {
                        if self.is_left(index) {
                            self.array[i].left = Some(j2);
                            if fixed {
                                self.array[j2].color = Black;
                                fixed = false;
                            }
                        } else {
                            self.array[i].right = Some(j2);
                            if fixed {
                                self.array[j2].color = Black;
                                fixed = false;
                            }
                        }
                        self.array[j2].parent = Some(i);
                    },
                    None => {
                        self.root = Some(j2);
                        self.array[j2].parent = None;
                        self.array[j2].color = Black;
                        fixed = false;
                    }
                }
            },
            (Some(j1), None) => {
                match self.array[index].parent {
                    Some(i) => {
                        if self.is_left(index) {
                            self.array[i].left = Some(j1);
                            if fixed {
                                self.array[j1].color = Black;
                                fixed = false;
                            }
                        } else {
                            self.array[i].right = Some(j1);
                            if fixed {
                                self.array[j1].color = Black;
                                fixed = false;
                            }
                        }
                        self.array[j1].parent = Some(i);
                    },
                    None => {
                        self.root = Some(j1);
                        self.array[j1].parent = None;
                        self.array[j1].color = Black;
                        fixed = false;
                    }
                }
            },
            (Some(j1), Some(j2)) => {

                let sucessor = self.sucessor(index).unwrap();

                let filho_sucessor = self.array[sucessor].right;

                (x, xp) = (Right, sucessor);

                fixed = if let Red = self.array[sucessor].color { false } else { true };

                self.array[sucessor].color = self.array[index].color;

                self.array[j1].parent = Some(sucessor);
                self.array[sucessor].left = Some(j1);

                let pai_sucessor = self.array[sucessor].parent.unwrap();

                match self.array[index].parent {
                    Some(i) => {
                        if self.is_left(index) {
                            self.array[i].left = Some(sucessor);
                        } else {
                            self.array[i].right = Some(sucessor);
                        }
                        self.array[sucessor].parent = Some(i);
                    },
                    None => {
                        self.root = Some(sucessor);
                        self.array[sucessor].parent = None;
                    }
                }

                if sucessor != j2 {

                    (x, xp) = (Left, pai_sucessor);

                    self.array[j2].parent = Some(sucessor);

                    self.array[sucessor].right = Some(j2);

                    match filho_sucessor {
                        Some(k) => {
                            self.array[pai_sucessor].left = Some(k);
                            self.array[k].parent = Some(pai_sucessor);
                        }
                        None => {
                            self.array[pai_sucessor].left = None;
                        }
                    }
                }
            }
        }

        let last = self.len() - 1;
        if index != last {
            match self.array[last].parent {
                Some(i) => {
                    if self.is_left(last) {
                        self.array[i].left = Some(index);
                    } else {
                        self.array[i].right = Some(index);
                    }
                },
                None => {
                    self.root = Some(index);
                }
            }
            match self.array[last].left {
                Some(i) => {
                    self.array[i].parent = Some(index)
                },
                None => {}
            }
            match self.array[last].right {
                Some(i) => {
                    self.array[i].parent = Some(index)
                },
                None => {}
            }
            self.array.swap(index, last);
        }

        if fixed {
            self.deletion_fixed_up(x, xp);
        }

        return Some(self.array.pop().unwrap().satelite)
    }

    fn is_black(self: &Self, x: Child, pai: usize) -> bool {

        match x  {
            Left => {
                match self.array[pai].left {
                    Some(i) => {
                        match self.array[i].color {
                            Red => false,
                            Black => true
                        }
                    }
                    None => true
                }
            }
            Right => {
                match self.array[pai].right {
                    Some(i) => {
                        match self.array[i].color {
                            Red => false,
                            Black => true
                        }
                    }
                    None => true
                }
            }
        }
    }

    fn deletion_fixed_up(self: &mut Self, mut x: Child, mut pai: usize) {

        let root = match self.root {
            Some(i) => i,
            None => return
        };

        let mut irmao;

        let mut condition = self.is_black(x, pai);

        if !condition {
            match x {   // se for vermelho, muda pra preto e nem entra no while
                Left => {
                    let left = self.array[pai].left.unwrap();
                    self.array[left].color = Black;
                }
                Right => {
                    let right = self.array[pai].right.unwrap();
                    self.array[right].color = Black;
                }
            }
        }

        while condition {  // se x se tornar root, colocamos como black e break loop. Senão continuamos.

            if let Left = x {

                irmao = self.array[pai].right.unwrap();  // enquanto index é black e não root, ele tem que ter irmao

                if let Red = self.array[irmao].color {  // logo ele tem dois filhos black, pois seu irmao é black
                    self.array[irmao].color = Black;
                    self.array[pai].color = Red;
                    self.left_rotate(pai);
                    irmao = self.array[pai].right.unwrap();  // é o antigo filho left do irmao, que é black
                }

                if self.is_black(Left, irmao) && self.is_black(Right, irmao) {
                    self.array[irmao].color = Red;
                    match self.array[pai].parent {
                        Some(i) => {
                            if self.is_left(pai) {
                                (x, pai) = (Left, i)
                            } else {
                                (x, pai) = (Right, i)
                            }
                            condition = self.is_black(x, pai);
                        }
                        None => {
                            self.array[pai].color = Black;
                            break;
                        }
                    }
                } else {

                    if self.is_black(Right, irmao) {   // logo o esquerdo é vermelho
                        let sobrinho_left = self.array[irmao].left.unwrap();
                        self.array[sobrinho_left].color = Black;
                        self.array[irmao].color = Red;
                        self.right_rotate(irmao);
                        irmao = self.array[pai].right.unwrap();  // é o antigo filho esquerdo de irmao
                    }
                    self.array[irmao].color = self.array[pai].color;
                    self.array[pai].color = Black;
                    // caso tenhamos passado pelo if acima, o filho direito de irmao é o antigo irmao,
                    // caso não tenhamos passado, é um vermelho .. logo não é Nil
                    let sobrinho_right = self.array[irmao].right.unwrap();
                    self.array[sobrinho_right].color = Black;
                    self.left_rotate(pai);
                    self.array[root].color = Black;
                    break;
                }
            } else {
                irmao = self.array[pai].left.unwrap();  // enquanto index é black e não root, ele tem que ter irmao

                if let Red = self.array[irmao].color {  // logo ele tem dois filhos black, pois seu irmao é black
                    self.array[irmao].color = Black;
                    self.array[pai].color = Red;
                    self.right_rotate(pai);
                    irmao = self.array[pai].left.unwrap();  // é o antigo filho rigth do irmao, que é black
                }

                if self.is_black(Left, irmao) && self.is_black(Right, irmao) {
                    self.array[irmao].color = Red;
                    match self.array[pai].parent {
                        Some(i) => {
                            if self.is_left(pai) {
                                (x, pai) = (Left, i)
                            } else {
                                (x, pai) = (Right, i)
                            }
                            condition = self.is_black(x, pai);
                        }
                        None => {
                            self.array[pai].color = Black;
                            break;
                        }
                    }
                } else {

                    if self.is_black(Left, irmao) {   // logo o direito é vermelho
                        let sobrinho_right = self.array[irmao].right.unwrap();
                        self.array[sobrinho_right].color = Black;
                        self.array[irmao].color = Red;
                        self.left_rotate(irmao);
                        irmao = self.array[pai].left.unwrap();  // é o antigo filho direito de irmao
                    }
                    self.array[irmao].color = self.array[pai].color;
                    self.array[pai].color = Black;
                    // caso tenhamos passado pelo if acima, o filho esquerdo de irmao é o antigo irmao,
                    // caso não tenhamos passado, é um vermelho .. logo não é Nil
                    let sobrinho_left = self.array[irmao].left.unwrap();
                    self.array[sobrinho_left].color = Black;
                    self.right_rotate(pai);
                    self.array[root].color = Black;
                    break;
                }
            }
        }
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

    pub fn get(self: &Self, value: &T) -> Option<&E> {

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

    pub fn get_mut(self: &mut Self, value: &T) -> Option<&mut E> {

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

    fn get_index(self: &Self, value: &T) -> Option<usize> {

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


    pub fn inorder(self: &Self) -> Vec<(&T,&E)> {

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
                            if self.array[j].value > self.array[index].value {
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

    fn indexes_subtree(self: &Self, index: usize) -> Vec<usize> {

        let mut w = vec![];

        match self.array[index].left {
            Some(i) => {
                let mut wl = self.indexes_subtree(i);
                w.append(&mut wl);
            }
            None => {}
        }

        w.push(index);

        match self.array[index].right {
            Some(i) => {
                let mut wr = self.indexes_subtree(i);
                w.append(&mut wr);
            }
            None => {}
        }
        w
    }

    fn counting_blacks(self: &Self, root: usize) -> bool {

        let indexes_subtree = self.indexes_subtree(root);

        let mut folhas = vec![];

        for &index in &indexes_subtree {
            match (&self.array[index].left, &self.array[index].right) {
                (None, None) => folhas.push(index),
                (None, Some(_)) => folhas.push(index),
                (Some(_), None) => folhas.push(index),
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

        counting.iter().min().unwrap() == counting.iter().max().unwrap()

    }

    fn red_not_parent_red(self: &Self) -> bool {

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

                if let (Red, Red) = (self.array[j].color, self.array[self.array[j].parent.unwrap()].color) {
                    return false
                } else {
                    j = self.array[j].parent.unwrap();
                    continue;
                }
            }
        }
        true
    }

    fn min_max_subtree_property(self: &Self, index: usize) -> bool {

        let max_left = match self.array[index].left {
            Some(i) => {
                let v = self.indexes_subtree(i);
                let mut max = v[0];
                for j in v {
                    if self.array[j].value > self.array[max].value {
                        max = j;
                    }
                }
                if self.array[max].value <= self.array[index].value { true } else { false }
            }
            None => true
        };

        let min_right = match self.array[index].right {
            Some(i) => {
                let v = self.indexes_subtree(i);
                let mut min = v[0];
                for j in v {
                    if self.array[j].value < self.array[min].value {
                        min = j;
                    }
                }
                if self.array[min].value >= self.array[index].value { true } else { false }
            }
            None => true
        };

        max_left && min_right

    }

    fn binary_tree_property(self: &Self) -> bool {

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

                let pai = self.array[j].parent.unwrap();

                if self.array[pai].value > self.array[j].value {

                    match self.array[pai].left {
                        Some(k) => {
                            if k != j {
                                return false
                            }
                        }
                        None => return false
                    }
                    j = pai;
                    continue;

                } else if self.array[pai].value < self.array[j].value {

                    match self.array[pai].right {
                        Some(k) => {
                            if k != j {
                                return false
                            }
                        }
                        None => return false
                    }
                    j = pai;
                    continue;

                } else {
                    j = pai;
                    continue;
                }
            }
        }
        true
    }
    fn root_is_black(self: &Self) -> bool {

        match self.root {
            Some(i) => if let Black = self.array[i].color { true } else { false },
            None => true
        }
    }

    pub fn is_red_black_tree(self: &Self) -> bool {

        if !self.root_is_black() {
            return false
        }

        if !self.red_not_parent_red() {
            return false
        }

        if !self.binary_tree_property() {
            return false
        }

        for index in 0..self.len() {
            if !self.counting_blacks(index) {
                return false
            }
        }

        for index in 0..self.len() {
            if !self.min_max_subtree_property(index) {
                return false
            }
        }
        true
    }
}

impl<T: Debug + PartialOrd, E> RedBlackTreeWithReps<T, E>  {
    pub fn print_elements(self: &Self) {

        for (i, element) in self.array.iter().enumerate() {
            println!("Index: {}, Valor: {:?}, Pai: {:?}, Left: {:?}, Right: {:?}, Cor: {:?}",
                     i, element.value, element.parent, element.left, element.right, element.color)
        }
    }
}