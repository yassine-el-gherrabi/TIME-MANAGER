import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { toast } from 'sonner';
import { UserForm } from '../../components/admin';
import { usersApi } from '../../api/users';
import { mapErrorToMessage } from '../../utils/errorHandling';
import type { CreateUserRequest } from '../../types/user';

export function CreateUserPage() {
  const navigate = useNavigate();
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState('');

  const handleSubmit = async (data: CreateUserRequest) => {
    setIsLoading(true);
    setError('');
    try {
      await usersApi.create(data);
      toast.success('User created successfully. Invitation email sent.');
      navigate('/admin/users');
    } catch (err) {
      setError(mapErrorToMessage(err));
    } finally {
      setIsLoading(false);
    }
  };

  const handleCancel = () => {
    navigate('/admin/users');
  };

  return (
    <div className="container mx-auto py-8 px-4 flex justify-center">
      <UserForm onSubmit={handleSubmit} onCancel={handleCancel} isLoading={isLoading} error={error} />
    </div>
  );
}
