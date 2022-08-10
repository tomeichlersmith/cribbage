# cribbage

Simulations of hands of the card game [cribbage](https://en.wikipedia.org/wiki/Cribbage).

I'm a analytical guy and I like playing cribbage. 
I want to know if there is a definite best strategy and I want to learn some rust.

[Wolfram Mathworld](https://mathworld.wolfram.com/Cribbage.html) has a page on cribbage;
however, it does not mention counting points from flushes. I still can use it as a good
point of comparison for making sure my scoring function is working as intended.

[Wikipedia](https://en.wikipedia.org/wiki/Cribbage_statistics) also has a page on cribbage
statistics which I can use as another point of comparison. Since v0.1.2 of this crate, the
`Hand::score` function has been validated against the table of frequencies from this site.

### To Do
- [ ] Functional hash and equality which does not care about order of cards in hand
- [ ] Random simulation between "players"
- [ ] Develop different potential stratgies for comparison
