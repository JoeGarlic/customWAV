#requires sox
#requires audacity
#requires python 3.8
#requires pytest-shutil

import os
import sox
import shutil

base_sentence = input("Enter your sentence: ") 
i = 0
cbn = sox.Combiner()
cbn.set_input_format(file_type=['wav', 'wav'])
cbn.convert(samplerate=44100, n_channels=2)

for word in base_sentence.split(): #start parsing through sentence
    filename = "words\\" + word + ".wav" #create instance of word.wav
    isEmpty = "words\\" + "_empty.wav"
    outfile = "words\\gen\\generatedSentence" + str(i) + ".wav"
    print(word)
    if i == 0:
        infiles = [isEmpty, filename ] #replace empty and filename with words
        cbn.build([infiles[0], infiles[1]], outfile, 'concatenate')
        print("first loop: " + word)

    else :
        prev_outfile = "words\\gen\\generatedSentence" + str(i-1) + ".wav"
        cbn.build([prev_outfile, filename], outfile, 'concatenate')
        print("loop: " + word)
    print("is done")
    i += 1

customPath = 'words/_custom.wav'
custom = "words\\_custom.wav"
isExist = os.path.exists(customPath)
print(isExist)
if isExist:
    os.remove(customPath)
os.rename(outfile, custom)




