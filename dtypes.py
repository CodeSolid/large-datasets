import numpy as np
dtypes = {
'row_id': np.int64,
 'timestamp': np.int32, 
 'user_id': np.int32, 
 'content_id': np.int32, 
 'content_type_id': np.int32,
'task_container_id': np.int32, 
'user_answer': np.int32, 
'answered_correctly': np.int32,
'prior_question_elapsed_time': np.float64, 
'prior_question_had_explanation': np.object
}