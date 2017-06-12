# markov
A flexible, all n-gram (bi-grams+) Markov chain-based text generator. Always looking for improvements, so suggest away!
## Notes on usage
The command-line implementation of this Python script calls for at least two, but up to three, parameters. They are:
* -t: specifies a text to train the chain. If a chain file already exists, new training appends to the current model.
* -g: # of sentences to generate.
* -n: n-gram model to use. This must be the same as the model order used to train (e.g., if training a trigram model (n=3), the -n flag must also be set to a trigram (n=3).
