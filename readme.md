# Cucchiaio

Photography is one of my hobbies, but I often take too many pictures, and I have to sort them afterwise.
This simple software can sort all the pictures in a folder in three categories : *Good*, *Bad* and *Ok*.

You can select the quality of each pcture with shortcuts and then they are moved into three subfolders.
I usualy delete the bad ones, edit the good ones strait away, and store the ok ones for whatever occasion.

### Running

The software is written in Rust, so you need cargo installed to run it.

	git clone https://gitlab.com/ddorn/cucchiaio
	cd cucchiaio
	cargo run

### Shortucts

There are very few shortcuts, but it i show you do everything.
 - `Ctrl+H` : Previous picture
 - `Ctrl+L` : Next picture
 - `Ctrl+J` : Flag as bad
 - `Ctrl+K` : Flag as bad
 - `Ctrl+R` : Remove flag / flag as neutral
 - `Ctrl+P`: Proccess all the changes (moves all images in the three subfolders, the app can be quit once the images disapear)
