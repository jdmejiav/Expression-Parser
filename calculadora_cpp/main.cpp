#include <iostream>
#include <string>
#include "scanner.h"

using namespace std;

int main (){

string in = "";
while (in!="exit"){
  cout << ">>> ";
  getline(cin,in);
  Scanner *sc = new Scanner(in);
  
}


}
