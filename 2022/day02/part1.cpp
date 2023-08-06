#include <fstream>
#include <iostream>
using namespace std;

int main(void) {
	int score = 0;
	char n, m;

	ifstream fin;
	fin.open("day02.txt");

	/**
	* A X
	* B Y
	* C Z
	**/

	while(fin >> n >> m) {
		if(m == 'X') {
			score += 1;

			if(n == 'A') score += 3;
			else if(n == 'C') score += 6;
		}
		else if(m == 'Y') {
			score += 2;

			if(n == 'B') score += 3;
			else if(n == 'A') score += 6;
		}
		else {
			score += 3;

			if(n == 'C') score += 3;
			else if(n == 'B') score += 6;
		}
	}

	fin.close();

	cout << score << endl;
}
