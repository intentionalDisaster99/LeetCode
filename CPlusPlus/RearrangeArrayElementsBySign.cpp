/*

    Author:     Sam Whitlock
    Date:       February 17, 2025

    Completed?  In Progress
    Notes:


*/
#include <iostream>
#include <vector>
using namespace std;

vector<int> rearrangeArray(vector<int> &nums);
vector<int> slowRearrangeArray(vector<int> &nums);

// Main function for testing
int main() {
  // Test vector
  vector<int> nums = {3, 1, -2, -5, 2, -4};

  // Testing new algorithm
  nums = rearrangeArray(nums);
  cout << "The output is: {";
  for (int i = 0; i < nums.size() - 1; i++) {
    cout << nums[i] << ", ";
  }
  cout << nums[nums.size() - 1] << "}" << endl;

  // Checking with one that I know works
  nums = {3, 1, -2, -5, 2, -4};
  nums = slowRearrangeArray(nums);
  cout << "The output should be is: {";
  for (int i = 0; i < nums.size() - 1; i++) {
    cout << nums[i] << ", ";
  }
  cout << nums[nums.size() - 1] << "}" << endl;
}

vector<int> rearrangeArray(vector<int> &nums) {
  bool signToggle = true;
  for (int i = 0; i < nums.size(); i++) {
    for (int j = i + 1; j < nums.size(); j++) {
      if (signToggle) {
        if (nums[j] <= 0) {
          continue;
        }
      } else {
        if (nums[j] >= 0) {
          continue;
        }
      }
      int tmp = nums[i];
      nums[i] = nums[j];
      nums[j] = tmp;
      break;
    }
    signToggle = !signToggle;
  }
  return nums;
}

// This works but is really slow
vector<int> slowRearrangeArray(vector<int> &nums) {
  vector<int> output(nums.size());

  bool signToggle = true;  // true is positive

  // O(n^2), the fastest algorithm ever
  for (int i = 0; i < output.size(); i++) {
    for (int j = 0; j < nums.size(); j++) {
      // Skipping if it is the wrong sign
      if (signToggle) {
        if (nums[j] <= 0) {
          continue;
        }
      } else {
        if (nums[j] >= 0) {
          continue;
        }
      }

      // Saving and resetting the og list
      output[i] = nums[j];
      nums.erase(nums.begin() + j);
      signToggle = !signToggle;
      break;
    }
  }
  return output;
}