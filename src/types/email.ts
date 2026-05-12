export interface EmailSummary {
  id: string;
  account_id: string;
  uid: number;
  subject: string;
  sender_name: string | null;
  sender_email: string | null;
  received_at: string | null;
  codes: string[];
  is_read: boolean;
}

export interface EmailDetail extends EmailSummary {
  body_text: string | null;
  body_html: string | null;
}

export interface AccountUnreadState {
  account_id: string;
  unread_count: number;
}

export interface NewMailEvent {
  accountId: string;
  emailId: string;
  subject: string;
  sender: string;
  code?: string | null;
  unreadCount: number;
}
