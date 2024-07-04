use leptos::*;
use leptos_meta::*;

#[component]
pub fn HelpPage() -> impl IntoView {
    view! {
        <Title text="Help"/>

        <main class="md:p-4">
            <div class="p-4 pb-8 mx-auto max-w-5xl bg-gray-50 border shadow-md">
                <header class="mb-4">
                    <h1 class="text-xl font-bold">"Help"</h1>
                </header>

                <nav class="mb-4">
                    <ul class="space-y-2">
                        <li>
                            <a href="#bmi" class="text-blue-500 hover:underline">
                                "Body Mass Index (BMI)"
                            </a>
                        </li>
                        <li>
                            <a href="#bmr" class="text-blue-500 hover:underline">
                                "Basal Metabolic Rate (BMR)"
                            </a>
                        </li>
                        <li>
                            <a href="#tdee" class="text-blue-500 hover:underline">
                                "Total Daily Energy Expenditure (TDEE)"
                            </a>
                        </li>
                    </ul>
                </nav>

                <section>
                    <article class="pt-16 space-y-2 leading-6" id="bmi">
                        <h2 class="text-xl font-bold">"Body Mass Index (BMI)"</h2>
                        <p>
                            "Body Mass Index (BMI) is a measure commonly used to assess a person's body weight in relation to their height. It's a simple calculation that provides an indication of whether an individual is underweight, normal weight, overweight, or obese, based on established ranges."
                        </p>

                        <p>"The formula for BMI is:"</p>

                        <p>"BMI = weight in kilograms / height in meters squared"</p>

                        <p>
                            "Alternatively, it can be calculated using pounds and inches with the formula:"
                        </p>

                        <p>"BMI = " "weight in pounds"/ "height in inches squared" " x 703"</p>

                        <p>"Here's what the BMI categories generally indicate:"</p>

                        <ol>
                            <li>
                                <strong>"Underweight: "</strong>
                                "BMI less than 18.5"
                            </li>
                            <li>
                                <strong>"Normal weight: "</strong>
                                "BMI between 18.5 and 24.9"
                            </li>
                            <li>
                                <strong>"Overweight: "</strong>
                                "BMI between 25 and 29.9"
                            </li>
                            <li>
                                <strong>"Obese:"</strong>
                                "BMI 30 or greater"
                            </li>
                        </ol>

                        <p>
                            "It's important to note that while BMI is a useful screening tool, it has limitations. For instance, it doesn't differentiate between fat and muscle mass, so a person with a high muscle mass may have a high BMI but not necessarily be overweight or obese. Additionally, BMI doesn't take into account factors such as age, sex, bone density, or distribution of fat, which are important in assessing overall health. Therefore, BMI should be interpreted alongside other health indicators and assessments for a more comprehensive evaluation of an individual's health status."
                        </p>

                    </article>

                    <article class="space-y-2 leading-6">
                        <h2 class="pt-16 text-xl font-bold" id="bmr">
                            "Basal Metabolic Rate (BMR)"
                        </h2>
                        <p>
                            "Basal Metabolic Rate (BMR) is the amount of energy expended by the body at rest to maintain basic physiological functions such as breathing, circulation, and cell production. It represents the minimum number of calories needed by the body to sustain life in a state of rest."
                        </p>

                        <p>
                            "BMR is influenced by various factors including age, sex, body composition, and genetics. Typically, BMR decreases with age and is generally higher in men than in women due to differences in muscle mass."
                        </p>

                        <p>
                            "Several formulas are used to estimate BMR, with the Harris-Benedict equation being one of the most widely used. This equation takes into account factors such as weight, height, age, and sex to calculate BMR."
                        </p>

                        <p>
                            "Understanding one's BMR can be helpful for designing an appropriate diet and exercise plan for weight management. It serves as a baseline for determining calorie needs, with adjustments made based on activity level and specific health goals."
                        </p>

                        <p>
                            "It's important to note that while BMR provides valuable insight into energy expenditure, it's just one component of overall metabolism. Other factors such as physical activity, thermic effect of food, and non-exercise activity thermogenesis (NEAT) also play significant roles in determining total daily energy expenditure."
                        </p>
                    </article>

                    <article class="space-y-2 leading-6">
                        <h2 class="pt-16 text-xl font-bold" id="tdee">
                            "Total Daily Energy Expenditure (TDEE)"
                        </h2>
                        <p>
                            "Total Daily Energy Expenditure (TDEE) represents the total number of calories that an individual burns in a day, taking into account all activities and bodily functions. It includes Basal Metabolic Rate (BMR) as well as additional calories expended through physical activity and the thermic effect of food."
                        </p>

                        <p>
                            "TDEE is influenced by various factors including age, sex, weight, height, body composition, and activity level. Individuals with higher muscle mass typically have a higher TDEE since muscle tissue burns more calories at rest compared to fat tissue."
                        </p>

                        <p>
                            "Calculating TDEE is important for understanding energy balance and managing weight. To estimate TDEE, one can use equations such as the Harris-Benedict equation or the Mifflin-St Jeor equation, which take into account factors like BMR, activity level, and age."
                        </p>

                        <p>
                            "Once TDEE is determined, individuals can adjust their calorie intake based on their health goals. To lose weight, one would typically consume fewer calories than their TDEE, creating a calorie deficit. Conversely, to gain weight, one would consume more calories than their TDEE, creating a calorie surplus."
                        </p>

                        <p>
                            "It's important to note that TDEE is an estimate and may vary depending on individual factors and lifestyle changes. Regular monitoring of weight and adjusting calorie intake accordingly can help individuals achieve their desired health outcomes."
                        </p>
                    </article>
                </section>
            </div>
        </main>
    }
}
