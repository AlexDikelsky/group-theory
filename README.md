This is a library for finiding explicit isomorphisms between groups, and creating
groups. It currently supports creating 

* [Dihedral](https://en.wikipedia.org/wiki/Dihedral_group)
* [Permutation](https://en.wikipedia.org/wiki/Permutation_group)
* [Alternating](https://en.wikipedia.org/wiki/Alternating_group)
* [Cyclic](https://en.wikipedia.org/wiki/Cyclic_group)
* [Multiplicitive mod n](https://en.wikipedia.org/wiki/Multiplicative_group_of_integers_modulo_n)

groups.

# Examples:
* Find isomorphisms between the multiplicitive group mod 13 and the cyclic group of order 12
```
for iso in multiplicitive(13).isomorphisms(&cyclic(12)) {
    println!("{:?}", &iso); 
}
```
* Find isomrphisms between alternating group with 4 elements and the dihedral group with 6 points
```
println!("{:?}", alternating(4).isomorphisms(&dihedral(6)));
```
