import java.util.HashMap;

public class Main {
    public static void main(String[] args) {
    }
}

class Solution {
    public ListNode removeZeroSumSublists(ListNode head) {
        ListNode dummy = new ListNode(0, head);
        HashMap<Integer, ListNode> sumMap = new HashMap<>();
        ListNode curr = dummy;
        int sum = 0;
        while (curr != null) {
            sum += curr.val;
            sumMap.put(sum, curr);
            curr = curr.next;
        }
        sum = 0;
        curr = dummy;
        while (curr != null) {
            sum += curr.val;
            curr.next = sumMap.get(sum).next;
            curr = curr.next;
        }
        return dummy.next;
    }
}

class ListNode {
    int val;
    ListNode next;
    ListNode() {}
    ListNode(int val) { this.val = val; }
    ListNode(int val, ListNode next) { this.val = val; this.next = next; }
}
