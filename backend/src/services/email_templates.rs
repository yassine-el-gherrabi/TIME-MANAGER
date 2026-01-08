/// Email templates for Time Manager application
/// All templates return HTML content for rich email formatting

/// Generate HTML email for user invitation
pub fn invite_template(first_name: &str, invite_url: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Welcome to Time Manager</title>
</head>
<body style="margin: 0; padding: 0; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif; background-color: #f4f4f5;">
    <table role="presentation" style="width: 100%; border-collapse: collapse;">
        <tr>
            <td align="center" style="padding: 40px 0;">
                <table role="presentation" style="width: 600px; max-width: 100%; border-collapse: collapse; background-color: #ffffff; border-radius: 8px; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);">
                    <tr>
                        <td style="padding: 40px 40px 20px; text-align: center;">
                            <h1 style="margin: 0; color: #18181b; font-size: 24px; font-weight: 600;">Welcome to Time Manager</h1>
                        </td>
                    </tr>
                    <tr>
                        <td style="padding: 0 40px 20px;">
                            <p style="margin: 0 0 16px; color: #3f3f46; font-size: 16px; line-height: 24px;">
                                Hello {first_name},
                            </p>
                            <p style="margin: 0 0 16px; color: #3f3f46; font-size: 16px; line-height: 24px;">
                                You've been invited to join Time Manager. Click the button below to set up your password and activate your account.
                            </p>
                        </td>
                    </tr>
                    <tr>
                        <td style="padding: 0 40px 30px; text-align: center;">
                            <a href="{invite_url}" style="display: inline-block; padding: 12px 32px; background-color: #18181b; color: #ffffff; text-decoration: none; font-size: 16px; font-weight: 500; border-radius: 6px;">
                                Activate Account
                            </a>
                        </td>
                    </tr>
                    <tr>
                        <td style="padding: 0 40px 20px;">
                            <p style="margin: 0; color: #71717a; font-size: 14px; line-height: 20px;">
                                If you didn't expect this invitation, you can safely ignore this email.
                            </p>
                        </td>
                    </tr>
                    <tr>
                        <td style="padding: 0 40px 20px;">
                            <p style="margin: 0; color: #71717a; font-size: 14px; line-height: 20px;">
                                This link will expire in 7 days.
                            </p>
                        </td>
                    </tr>
                    <tr>
                        <td style="padding: 20px 40px; border-top: 1px solid #e4e4e7;">
                            <p style="margin: 0; color: #a1a1aa; font-size: 12px; line-height: 18px; text-align: center;">
                                &copy; Time Manager. All rights reserved.
                            </p>
                        </td>
                    </tr>
                </table>
            </td>
        </tr>
    </table>
</body>
</html>"#,
        first_name = first_name,
        invite_url = invite_url
    )
}

/// Generate HTML email for password reset
pub fn password_reset_template(first_name: &str, reset_url: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Reset Your Password</title>
</head>
<body style="margin: 0; padding: 0; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif; background-color: #f4f4f5;">
    <table role="presentation" style="width: 100%; border-collapse: collapse;">
        <tr>
            <td align="center" style="padding: 40px 0;">
                <table role="presentation" style="width: 600px; max-width: 100%; border-collapse: collapse; background-color: #ffffff; border-radius: 8px; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);">
                    <tr>
                        <td style="padding: 40px 40px 20px; text-align: center;">
                            <h1 style="margin: 0; color: #18181b; font-size: 24px; font-weight: 600;">Reset Your Password</h1>
                        </td>
                    </tr>
                    <tr>
                        <td style="padding: 0 40px 20px;">
                            <p style="margin: 0 0 16px; color: #3f3f46; font-size: 16px; line-height: 24px;">
                                Hello {first_name},
                            </p>
                            <p style="margin: 0 0 16px; color: #3f3f46; font-size: 16px; line-height: 24px;">
                                We received a request to reset your password. Click the button below to choose a new password.
                            </p>
                        </td>
                    </tr>
                    <tr>
                        <td style="padding: 0 40px 30px; text-align: center;">
                            <a href="{reset_url}" style="display: inline-block; padding: 12px 32px; background-color: #18181b; color: #ffffff; text-decoration: none; font-size: 16px; font-weight: 500; border-radius: 6px;">
                                Reset Password
                            </a>
                        </td>
                    </tr>
                    <tr>
                        <td style="padding: 0 40px 20px;">
                            <p style="margin: 0; color: #71717a; font-size: 14px; line-height: 20px;">
                                If you didn't request a password reset, you can safely ignore this email. Your password will remain unchanged.
                            </p>
                        </td>
                    </tr>
                    <tr>
                        <td style="padding: 0 40px 20px;">
                            <p style="margin: 0; color: #71717a; font-size: 14px; line-height: 20px;">
                                This link will expire in 1 hour.
                            </p>
                        </td>
                    </tr>
                    <tr>
                        <td style="padding: 20px 40px; border-top: 1px solid #e4e4e7;">
                            <p style="margin: 0; color: #a1a1aa; font-size: 12px; line-height: 18px; text-align: center;">
                                &copy; Time Manager. All rights reserved.
                            </p>
                        </td>
                    </tr>
                </table>
            </td>
        </tr>
    </table>
</body>
</html>"#,
        first_name = first_name,
        reset_url = reset_url
    )
}

/// Generate plain text version of invite email (for email clients that don't support HTML)
pub fn invite_template_plain(first_name: &str, invite_url: &str) -> String {
    format!(
        r#"Welcome to Time Manager

Hello {first_name},

You've been invited to join Time Manager. Visit the link below to set up your password and activate your account:

{invite_url}

If you didn't expect this invitation, you can safely ignore this email.

This link will expire in 7 days.

---
Time Manager"#,
        first_name = first_name,
        invite_url = invite_url
    )
}

/// Generate plain text version of password reset email
pub fn password_reset_template_plain(first_name: &str, reset_url: &str) -> String {
    format!(
        r#"Reset Your Password

Hello {first_name},

We received a request to reset your password. Visit the link below to choose a new password:

{reset_url}

If you didn't request a password reset, you can safely ignore this email. Your password will remain unchanged.

This link will expire in 1 hour.

---
Time Manager"#,
        first_name = first_name,
        reset_url = reset_url
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invite_template_contains_name_and_url() {
        let html = invite_template("John", "https://example.com/invite?token=abc");
        assert!(html.contains("John"));
        assert!(html.contains("https://example.com/invite?token=abc"));
        assert!(html.contains("Activate Account"));
    }

    #[test]
    fn test_password_reset_template_contains_name_and_url() {
        let html = password_reset_template("Jane", "https://example.com/reset?token=xyz");
        assert!(html.contains("Jane"));
        assert!(html.contains("https://example.com/reset?token=xyz"));
        assert!(html.contains("Reset Password"));
    }

    #[test]
    fn test_plain_templates() {
        let plain = invite_template_plain("John", "https://example.com/invite");
        assert!(plain.contains("John"));
        assert!(plain.contains("https://example.com/invite"));

        let plain = password_reset_template_plain("Jane", "https://example.com/reset");
        assert!(plain.contains("Jane"));
        assert!(plain.contains("https://example.com/reset"));
    }
}
