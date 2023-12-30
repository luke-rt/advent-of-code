#include <fstream>
#include <iostream>

int main(void) {
	std::ifstream fin;
	fin.open("input.txt");

	std::string line;
	while(fin >> line) {

	}

	fin.close();
}
