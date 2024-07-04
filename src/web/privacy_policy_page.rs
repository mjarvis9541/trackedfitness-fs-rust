use leptos::*;
use leptos_meta::*;

#[component]
pub fn PrivacyPage() -> impl IntoView {
    view! {
        <Title text="Privacy Policy"/>

        <div class="container mx-auto p-6 bg-white shadow-md rounded-lg text-base">
            <h1 class="text-3xl font-bold mb-4">"Privacy Policy"</h1>
            <p class="mb-4">
                <strong>"Effective Date: "</strong>
                "14 June 2024"
            </p>

            <h2 class="text-2xl font-semibold mb-2">"1. Introduction"</h2>
            <p class="mb-4">
                "Welcome to Trackedfitness, your ultimate fitness companion. At Trackedfitness, we prioritize your privacy and are committed to protecting your personal information. This Privacy Policy outlines how we collect, use, and safeguard your data when you use our app. By using Trackedfitness, you agree to the practices described in this policy."
            </p>

            <h2 class="text-2xl font-semibold mb-2">"2. Data We Collect"</h2>
            <p class="mb-4">
                "When you use Trackedfitness, we collect the following information to enhance your experience: "
            </p>
            <ul class="list-disc list-inside mb-4">
                <li>
                    <strong>"Personal Information: "</strong>
                </li>
                <ul class="list-disc list-inside ml-6 mb-4">
                    <li>"Name"</li>
                    <li>"Email (never shared)"</li>
                    <li>
                        "Date of Birth (DOB) (never shared, used only for Basal Metabolic Rate (BMR) calculations)"
                    </li>
                    <li>
                        "Fitness Goals (e.g., lose weight, maintain weight, gain weight) (optionally shared)"
                    </li>
                    <li>"Height (never shared)"</li>
                    <li>"Weight (optionally shared)"</li>
                    <li>"Activity Level (optionally shared)"</li>
                </ul>
            </ul>

            <h2 class="text-2xl font-semibold mb-2">"3. Features and Their Data Usage"</h2>
            <ul class="list-disc list-inside mb-4">
                <li>
                    <strong>"Diet Targets: "</strong>
                    "Set daily calorie and macro targets based on your fitness goals, height, weight, and activity level. (Optionally shared)"
                </li>
                <li>
                    <strong>"Saved Meals: "</strong>
                    "Create and save meals/recipes to streamline your nutrition logging. (Optionally shared)"
                </li>
                <li>
                    <strong>"Nutrition Log: "</strong>
                    "Log daily food intake and track nutrients to stay on top of your diet. (Optionally shared)"
                </li>
                <li>
                    <strong>"Training Plans: "</strong>
                    "Select from pre-defined workout routines or create and share your own tailored to your fitness level and goals."
                </li>
                <li>
                    <strong>"Training Log: "</strong>
                    "Log and track workouts to monitor progress and adjust plans as needed. (Optionally shared)"
                </li>
                <li>
                    <strong>"Progress Journal: "</strong>
                    "Log notes on feelings, mindfulness, body weight, energy burnt, and photos to document your journey. (Optionally shared)"
                </li>
                <li>
                    <strong>"Social Hub: "</strong>
                    "Follow users, share plans, logs, and progress with the Trackedfitness community."
                </li>
                <li>
                    <strong>"Fitness Profile: "</strong>
                    "Track metrics like weight and goals to stay motivated and informed. (Optionally shared)"
                </li>
            </ul>

            <h2 class="text-2xl font-semibold mb-2">"4. How We Use Your Data"</h2>
            <p class="mb-4">"We use your data to: "</p>
            <ul class="list-disc list-inside mb-4">
                <li>"Personalize your fitness and nutrition experience."</li>
                <li>"Provide accurate BMR calculations and diet targets."</li>
                <li>
                    "Enable social interactions and sharing within the Trackedfitness community."
                </li>
                <li>"Improve app functionality and user experience."</li>
                <li>"Communicate with you about updates, features, and support."</li>
            </ul>

            <h2 class="text-2xl font-semibold mb-2">"5. Data Sharing and Disclosure"</h2>
            <p class="mb-4">
                "We prioritize your privacy and ensure that your data is shared only in the following circumstances: "
            </p>
            <ul class="list-disc list-inside mb-4">
                <li>
                    <strong>"Personal Information: "</strong>
                    "Your name, email, DOB, and height are never shared."
                </li>
                <li>
                    <strong>
                        "Fitness Goals, Weight, Activity Level, Diet Targets, Saved Meals, Nutrition Log, Training Log, and Progress Journal: "
                    </strong>
                    "These can be optionally shared within the app's social features."
                </li>
                <li>
                    <strong>"Legal Requirements: "</strong>
                    "We may disclose your information if required by law or in response to legal processes."
                </li>
            </ul>

            <h2 class="text-2xl font-semibold mb-2">"6. User Controls and Privacy Settings"</h2>
            <ul class="list-disc list-inside mb-4">
                <li>
                    <strong>"User Blocking and Reporting: "</strong>
                    "You can block or report users to ensure a safe and respectful environment."
                </li>
                <li>
                    <strong>"Follower System: "</strong>
                    "You control who can follow you and view your shared content."
                </li>
            </ul>

            <h2 class="text-2xl font-semibold mb-2">"7. Data Security"</h2>
            <p class="mb-4">
                "We implement robust security measures to protect your personal information from unauthorized access, alteration, disclosure, or destruction. However, please note that no method of transmission over the internet or electronic storage is 100% secure."
            </p>

            <h2 class="text-2xl font-semibold mb-2">"8. Your Rights"</h2>
            <p class="mb-4">"You have the right to: "</p>
            <ul class="list-disc list-inside mb-4">
                <li>"Access and update your personal information."</li>
                <li>"Delete your account and personal data."</li>
                <li>
                    "Control the sharing of your fitness goals, weight, activity level, diet targets, saved meals, nutrition log, training log, and progress journal."
                </li>
            </ul>

            <h2 class="text-2xl font-semibold mb-2">"9. Changes to This Privacy Policy"</h2>
            <p class="mb-4">
                "We may update this Privacy Policy from time to time. We will notify you of any significant changes by posting the new policy on our app and updating the effective date."
            </p>

            <h2 class="text-2xl font-semibold mb-2">"10. Contact Us"</h2>
            <p class="mb-4">
                "If you have any questions or concerns about this Privacy Policy, please contact us at: "
            </p>
            <div class="contact-info mb-4">
                <p>
                    <strong>"Email: "</strong>
                    "privacy@trackedfitness.com"
                </p>
                <p>
                    <strong>"Address: "</strong>
                    "Trackedfitness Inc."
                </p>
            </div>

            <p class="mb-4">
                "By using Trackedfitness, you acknowledge that you have read, understood, and agreed to this Privacy Policy."
            </p>

            <p class="mb-4">
                <strong>"Last Updated: "</strong>
                "14 June 2024"
            </p>
        </div>
    }
}
