#include <fstream>
#include <ostream>

int main(void) {
	std::ifstream fin;
	fin.open("day00.txt");

	std::string line;
	while(fin >> line) {

	}

	fin.close();
}

