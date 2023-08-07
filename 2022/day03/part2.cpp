#include <fstream>
#include <iostream>

int main(void) {
	int total = 0;

	std::ifstream fin;
	fin.open("day03.txt");

	std::string a, b, c;

	while(fin >> a >> b >> c) {
		for(char ch : a) {
			if(b.find(ch) != std::string::npos && c.find(ch) != std::string::npos) {
				if(islower(ch)) total += int(ch) - 96;
				else total += int(ch) - 38;
				break;
			}
		}
	}

	std::cout << total;
}
