use leptos::*;
use leptos_meta::*;

#[component]
pub fn TermsPage() -> impl IntoView {
    view! {
        <Title text="Terms of Service"/>
        <div class="container mx-auto p-6 bg-white shadow-md rounded-lg text-base">
            <h1 class="text-3xl font-bold mb-4">"Terms of Service"</h1>
            <p class="mb-4">
                <strong>"Effective Date: "</strong>
                "14 June 2024"
            </p>

            <h2 class="text-2xl font-semibold mb-2">"1. Introduction"</h2>
            <p class="mb-4">
                "Welcome to Trackedfitness. By accessing or using our app, you agree to be bound by these Terms of Service (\"Terms\"). If you do not agree to these Terms, please do not use our app."
            </p>

            <h2 class="text-2xl font-semibold mb-2">"2. Use of the App"</h2>
            <p class="mb-4">
                "You must be at least 13 years old to use Trackedfitness. By using the app, you represent and warrant that you meet this age requirement."
            </p>

            <h2 class="text-2xl font-semibold mb-2">"3. Account Registration"</h2>
            <p class="mb-4">
                "To access certain features of the app, you may be required to register for an account. You agree to provide accurate, current, and complete information during the registration process and to update such information to keep it accurate, current, and complete."
            </p>

            <h2 class="text-2xl font-semibold mb-2">"4. User Conduct"</h2>
            <p class="mb-4">
                "You agree not to use the app for any unlawful purpose or in any way that could harm, disable, overburden, or impair the app. You also agree not to interfere with the security or proper functioning of the app or with any other user's enjoyment of the app."
            </p>

            <h2 class="text-2xl font-semibold mb-2">"5. Content Sharing"</h2>
            <p class="mb-4">
                "You are responsible for all content that you upload, post, or otherwise transmit via the app. You grant Trackedfitness a non-exclusive, royalty-free, worldwide license to use, distribute, reproduce, modify, adapt, publicly perform, and publicly display such content."
            </p>

            <h2 class="text-2xl font-semibold mb-2">"6. Intellectual Property"</h2>
            <p class="mb-4">
                "All content, trademarks, and data on the app, including but not limited to software, databases, text, graphics, icons, and hyperlinks, are the property of or licensed to Trackedfitness and are protected by law. Unauthorized use of this content is prohibited."
            </p>

            <h2 class="text-2xl font-semibold mb-2">"7. Privacy Policy"</h2>
            <p class="mb-4">
                "Your use of the app is also governed by our Privacy Policy. Please review our Privacy Policy to understand our practices regarding your personal information."
            </p>

            <h2 class="text-2xl font-semibold mb-2">"8. Termination"</h2>
            <p class="mb-4">
                "We may terminate or suspend your account and access to the app at our sole discretion, without prior notice or liability, for any reason, including if you breach these Terms."
            </p>

            <h2 class="text-2xl font-semibold mb-2">"9. Disclaimer of Warranties"</h2>
            <p class="mb-4">
                "The app is provided on an \"as is\" and \"as available\" basis. Trackedfitness makes no representations or warranties of any kind, express or implied, as to the operation of the app or the information, content, or materials included on the app."
            </p>

            <h2 class="text-2xl font-semibold mb-2">"10. Limitation of Liability"</h2>
            <p class="mb-4">
                "To the fullest extent permitted by law, Trackedfitness shall not be liable for any damages of any kind arising from the use of the app or from any information, content, or materials included on the app."
            </p>

            <h2 class="text-2xl font-semibold mb-2">"11. Changes to These Terms"</h2>
            <p class="mb-4">
                "We may modify these Terms at any time. We will notify you of any changes by posting the new Terms on our app. You are advised to review these Terms periodically for any changes."
            </p>

            <h2 class="text-2xl font-semibold mb-2">"12. Contact Us"</h2>
            <p class="mb-4">
                "If you have any questions about these Terms, please contact us at: "
            </p>
            <div class="contact-info mb-4">
                <p>
                    <strong>"Email: "</strong>
                    "support@trackedfitness.com"
                </p>
                <p>
                    <strong>"Address: "</strong>
                    "Trackedfitness Inc."
                </p>
            </div>

            <p class="mb-4">
                "By using Trackedfitness, you acknowledge that you have read, understood, and agreed to these Terms of Service."
            </p>

            <p class="mb-4">
                <strong>"Last Updated: "</strong>
                "14 June 2024"
            </p>
        </div>
    }
}
