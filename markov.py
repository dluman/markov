import getopt, numpy, pickle, random, re, string, sys

class Console:

	def __init__(self):
		self.i = 0

	def update(self,total,i):
		pct = i/total
		sys.stdout.write("Progress: %d / %d \r" % (i, total))
		sys.stdout.flush()

class File:

	def parse(self, n, text, marker):
		strings = []
		sentences = re.findall(r'[^.!\?]+[\.!\?]',text)
		for sentence in sentences:
			sentence = re.sub('"','',sentence)
			sentence = re.sub("'","",sentence)
			strings.append(marker + sentence)
		return strings
	
	def save(self, chain):
		pickle.dump(chain,open("chain.p","wb"))
	
	def load(self):
		chain = pickle.load(open("chain.p","rb"))
		return chain

class Chain:
	
	def model(self, n, ngram, state, text):
		for gram in ngram:
			key, val = [gram[i:n-1] for i in range(n-1)][0], gram[n-1]
			try: state[key].append(val)
			except KeyError: state[key] = [val]
		return state

	def generatetext(self, n, chain):
		text, sentence = [], []
		markers = list('ABCDEFGHIJKLNMOPQRSTUVWXYZ')
		marker = [markers[i] + markers[i] for i in range(n-1)]
		wx = marker[len(marker)-1]
		punct = set('!?.')
		tx = marker[:len(marker)-1]
		if n >= 3: tx = tuple(tx) + tuple([wx])
		else: tx = tuple([wx])
		while True:
			tx, wx = wx, random.choice(chain[tx])
			if n < 3: tx = tuple([tx])
			elif n == 3: tx = tuple([tx]) + tuple([wx])
			elif n > 3:
				tx = random.sample(list(key for key, val in chain.items() if wx in val),1)[0]
				tx = tx[n-(n-1):] + tuple([wx])
			if wx[len(wx)-1] in punct:
				sentence.append(wx)
				break
			sentence.append(wx)
		return sentence
		
class Ngram:
	
	def gram(self, n, text):
		if len(text) < n: return
		return zip(*[text[i:(len(text)+(n-1))-i] for i in xrange(n)])
		
def train(t, n):
	state, output = dict(), []
	text = open(t,'rb').read()
	
	markers = list('ABCDEFGHIJKLMNOPQRSTUVWXYZ')
	marker = ''.join([markers[i] + markers[i] + ' ' for i in range(n-1)])

	sentences = File().parse(n, text, marker)

	try: state = File().load()
	except IOError: pass
	i, total = 0, len(sentences)
	for sentence in sentences:
		words = sentence.split()
		ngram = Ngram().gram(n, words)
		if ngram is not None: state = Chain().model(n, ngram, state, text)
		Console().update(total, i)
		i += 1
	File().save(state)

def generate(g, n):
	output = []
	try: state = File().load()
	except IOError: sys.exit(2)
	for x in xrange(g):
		output.append(' '.join(Chain().generatetext(n, state)))
	return ' '.join(output)

def main(argv):
	t, g, n = None, None, None
	try:
		opts, args = getopt.getopt(argv,'t:g:n:',['t=','g=','n='])
	except getopt.GetoptError:
		print 'markov.py -t <text to train> -g <# of sentences to generate> -n <markov order>'
		sys.exit(2)
	for opt, arg in opts:
		if opt == '-h':
			print 'markov.py -t <text to train> -g <# of sentences to generate> -n <markov order>'
			sys.exit(2)
		elif opt in ('-t', '--t'):
			t = arg
		elif opt in ('-g', '--g'):
			g = int(arg)
		elif opt in ('-n', '--n'):
			n = int(arg)
		else:
			print 'markov.py -t <text to train> -g <# of sentences to generate> -n <markov order>'
			sys.exit(2)

	if t and n: train(t,n)
	elif g and n: print generate(g, n)
	else: sys.exit(2)

if __name__=='__main__': main(sys.argv[1:])