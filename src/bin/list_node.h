struct ListNode {
  int val;
  struct ListNode *next;
};

void print_list(struct ListNode *head);

struct ListNode *create_list(int n);

struct ListNode *create_list_from_to(int from, int to);

struct ListNode *create_list_from_array(int ns[], int n);
