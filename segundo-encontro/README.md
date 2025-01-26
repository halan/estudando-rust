# Assuntos mencionados

- **Structs**: Estruturas em Rust são formas de agrupar dados relacionados. São semelhantes a “classes” em outras linguagens, mas sem métodos diretamente associados. São definidas com a palavra-chave struct e podem conter diferentes tipos de dados.
- **Tipos básicos**: Rust possui tipos básicos como inteiros (i8, u8, i32, etc.), números de ponto flutuante (f32, f64), booleanos (bool). Não falamos muitos dos tipos caracteres (char), strings (String, &str), mas usamos brevemente.
- **Arrays**: Arrays são coleções de elementos com tamanho fixo e do mesmo tipo. Definidos com colchetes ([ ]), os arrays são úteis quando o número de elementos é conhecido em tempo de compilação.
- **Vetores**: Vetores (Vec<T>) são coleções dinâmicas que podem crescer ou diminuir em tamanho. São amplamente usados em Rust para armazenar elementos em listas redimensionáveis.
- **Tuplas**: Tuplas são grupos de valores que podem ter tipos diferentes. São úteis para retornar múltiplos valores de uma função ou agrupar dados temporariamente. Falamos também de unity ()
- **Enums**: Enumeradores permitem representar um conjunto de variantes nomeadas, cada uma podendo conter dados adicionais. Eles são úteis para modelar opções e estados de forma clara e segura. (https://doc.rust-lang.org/book/ch06-00-enums.html)
- **Iteradores**: Iteradores em Rust fornecem uma maneira eficiente e funcional de processar sequências de dados. Eles permitem percorrer coleções como arrays e vetores com métodos como map, filter e collect. Não fomos muito fundo só mostramos uma breve correlação com o for tradicional. (https://doc.rust-lang.org/book/ch13-02-iterators.html)
- **Módulos**: Módulos (mod) são usados para organizar o código em Rust, controlando escopo e privacidade. Eles ajudam a dividir um programa em várias partes reutilizáveis e legíveis. (https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html)
- **Traits**: Traits são similares a interfaces em outras linguagens. Elas definem um conjunto de métodos que podem ser implementados por diferentes tipos, permitindo polimorfismo e extensibilidade. (https://doc.rust-lang.org/book/ch10-02-traits.html)
- **Box**: Box<T> é usado para alocar valores no heap em vez da stack. É útil para criar tipos de tamanho indeterminado ou reduzir o tamanho de structs grandes. (https://doc.rust-lang.org/book/ch15-01-box.html)
- **Trait Objects**: Objetos de trait permitem o uso de polimorfismo dinâmico em Rust. Usando a palavra-chave dyn, você pode armazenar diferentes tipos que implementam a mesma trait. (https://doc.rust-lang.org/book/ch17-02-trait-objects.html e https://doc.rust-lang.org/std/keyword.dyn.html)
- **Lifetime**: Lifetimes gerenciam a validade de referências no Rust, garantindo segurança sem necessidade de garbage collection. Eles asseguram que uma referência nunca será usada após o dado ao qual aponta ser desalocado. (https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)

# Dramas

- Apanhamos um pouco pra entender que o dyn trait estava assumindo uma lifetime implícita como `'static` (https://chatgpt.com/share/67964cdb-f428-800b-9ca1-d6b97c7d74a9 e https://doc.rust-lang.org/reference/types/trait-object.html#trait-object-lifetime-bounds)
