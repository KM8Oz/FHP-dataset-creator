import pandas as pd
from sklearn.model_selection import train_test_split
from sklearn.ensemble import RandomForestClassifier
from sklearn.metrics import accuracy_score
from sklearn.externals import joblib

# Load data from CSV file
df = pd.read_csv('tracked_data.csv', header=None, names=['name', 'network_bytes_sent', 'network_bytes_received', 'disk_bytes_written', 'cpu_usage', 'active'])

# Get unique software names
software_names = df['name'].unique()

# Loop through software names and train models
for software_name in software_names:
    # Extract data for current software name
    software_df = df[df['name'] == software_name]
    X = software_df.iloc[:, 1:-1].values
    y = software_df.iloc[:, -1].values

    # Split data into training and testing sets
    X_train, X_test, y_train, y_test = train_test_split(X, y, test_size=0.2, random_state=42)

    # Train a random forest classifier
    clf = RandomForestClassifier(n_estimators=100, random_state=42)
    clf.fit(X_train, y_train)

    # Save the trained model
    joblib.dump(clf, f'{software_name}_model.pkl')

    # Make predictions on the test set
    y_pred = clf.predict(X_test)

    # Calculate accuracy score
    accuracy = accuracy_score(y_test, y_pred)
    print(f'{software_name} Accuracy: {accuracy}')

# Example prediction on new data (replace with actual tracked data)
new_data = [['app1', 100, 50, 10000, 20], ['app2', 0, 0, 0, 0]]
for data in new_data:
    software_name = data[0]
    tracked_data = data[1:]
    clf = joblib.load(f'{software_name}_model.pkl')
    prediction = clf.predict([tracked_data])
    print(f'{software_name} Prediction: {prediction}')
