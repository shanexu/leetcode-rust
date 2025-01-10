public class Main {}

class Solution {
    public ListNode modifiedList(int[] nums, ListNode head) {
        java.util.BitSet set = new java.util.BitSet(100001);
        for (int n : nums) {
            set.set(n);
        }
        ListNode dummy = new ListNode(0, head);
        ListNode curr = dummy;
        while (curr.next != null) {
            if (set.get(curr.next.val)) {
                curr.next = curr.next.next;
            } else {
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

