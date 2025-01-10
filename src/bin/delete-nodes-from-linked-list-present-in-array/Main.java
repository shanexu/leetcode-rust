public class Main {}

class Solution {
    public ListNode modifiedList(int[] nums, ListNode head) {
        java.util.BitSet set = new java.util.BitSet(100001);
        for (int n : nums) {
            set.set(n);
        }
        ListNode dummy = new ListNode(0, head);
        ListNode prev = dummy;
        ListNode curr = head;
        while (curr != null) {
            if (set.get(curr.val)) {
                prev.next = curr.next;
                curr = prev.next;
            } else {
                prev = curr;
                curr = curr.next;
            }
        }
        return dummy.next;
    }
}

class ListNode {
    int val;
    ListNode next;

    ListNode() {
    }

    ListNode(int val) {
        this.val = val;
    }

    ListNode(int val, ListNode next) {
        this.val = val;
        this.next = next;
    }
}

