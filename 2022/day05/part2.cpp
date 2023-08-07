#include <fstream>
#include <iostream>
#include <stack>

#define ROWS 8
#define COLS 9

int main(void) {
	char temp[COLS][ROWS];

	std::ifstream fin;
	fin.open("day05.txt");

	// crate structure
	std::string line;
	for(int j = 0; j < ROWS; ++j) {
		std::getline(fin, line);
		for(int i = 0; i < COLS; ++i) {
			temp[i][j] = line.at((i*4)+1);
		}
	}

	// creates stack from input data
	std::stack<char> stacks[COLS];
	for(int i = 0; i < COLS; ++i) {
		for(int j = ROWS-1; j >= 0; --j) {
			if(temp[i][j] == ' ') break;
			stacks[i].push(temp[i][j]);
		}
	}

	std::getline(fin, line);
	std::getline(fin, line);

	// instructions
	std::string _, q, a, b;
	while(fin >> _ >> q >> _ >> a >> _ >> b) {
		int quantity = stoi(q);
		// -1 to start indexing from 0
		int start = stoi(a) - 1;
		int end = stoi(b) - 1;

		std::stack<char> tmp;

		for(int i = 0; i < quantity; ++i) {
			tmp.push(stacks[start].top());
			stacks[start].pop();
		}
		while(!tmp.empty()) {
			stacks[end].push(tmp.top());
			tmp.pop();
		}
	}

	fin.close();

	for(int i = 0; i < COLS; ++i) {
		std::cout << stacks[i].top();
	}
}
