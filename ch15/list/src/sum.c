int test_sum(int* a, int n) {
  int sum = 0;
  for (int i = 0; i < n; ++i) {
    sum += a[i];
  }
  return sum;
}

int TestDemo(int* a, int len) {
  int i = 0;
  int sum = 0;
  // __asm {
  //   mov eax,i
  //   mov ecx, a
  //   mov edx, sum
  // _loop:
  //   cmp eax, len
  //   jre _loop
  //   add edx, [ecx+eax*4]
  //   inc eax
  //   jmp _loop
  // _End_loop:
  //   mov sum,edx
  // }
  return sum;
}

int* quick_sort(int* arr, int len) {
  if (len <= 1) {
    return arr;
  }
  int pivot = arr[0];
  int* left = (int*)malloc(sizeof(int) * len);
  int* right = (int*)malloc(sizeof(int) * len);
  int left_len = 0;
  int right_len = 0;
  for (int i = 1; i < len; ++i) {
    if (arr[i] < pivot) {
      left[left_len++] = arr[i];
    } else {
      right[right_len++] = arr[i];
    }
  }
  left = quick_sort(left, left_len);
  right = quick_sort(right, right_len);
  int* result = (int*)malloc(sizeof(int) * len);
  memcpy(result, left, sizeof(int) * left_len);
  result[left_len] = pivot;
  memcpy(result + left_len + 1, right, sizeof(int) * right_len);
  free(left);
  free(right);
  return result;
}